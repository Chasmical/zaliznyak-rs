use crate::{
    util::{InflectionBuf, StackBuf},
    word::{Utf8Letter, Utf8LetterSlice},
};

pub(super) type Pos = u8;

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub(super) struct WordInfo {
    pub(super) len: Pos,
    pub(super) stem_len: Pos,
    pub(super) insert_stress_pos: Pos,
}

#[derive_const(Default)]
pub struct WordBuf {
    // Declinable parts of Russian words very rarely exceed 15 letters
    pub(super) buf: StackBuf<Utf8Letter, 15>,
    pub(super) info: WordInfo,
}

#[derive(Copy)]
#[derive_const(Clone)]
pub struct Word<'a> {
    pub(super) buf: &'a Utf8Letter,
    pub(super) info: WordInfo,
}

impl WordInfo {
    pub const fn entire<'a>(&self, slice: &'a Utf8Letter) -> &'a [Utf8Letter] {
        unsafe { std::slice::from_raw_parts(slice, self.len as _) }
    }
    pub const fn stem<'a>(&self, slice: &'a Utf8Letter) -> &'a [Utf8Letter] {
        unsafe { std::slice::from_raw_parts(slice, self.stem_len as _) }
    }
    pub const fn ending<'a>(&self, slice: &'a Utf8Letter) -> &'a [Utf8Letter] {
        let start = unsafe { (&raw const *slice).add(self.stem_len as _) };
        unsafe { std::slice::from_raw_parts(start, (self.len - self.stem_len) as _) }
    }
}

impl WordBuf {
    pub(crate) fn with_capacity_for(stem: &str) -> Self {
        Self::with_capacity(InflectionBuf::max_char_len_for_noun(stem.len()))
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self { buf: StackBuf::with_capacity(cap), info: Default::default() }
    }

    pub fn is_empty(&self) -> bool {
        self.info.len == 0
    }
    pub const fn as_letters(&self) -> &[Utf8Letter] {
        self.info.entire(unsafe { &*self.buf.as_ptr() })
    }
    pub const fn stem_letters(&self) -> &[Utf8Letter] {
        self.info.stem(unsafe { &*self.buf.as_ptr() })
    }
    pub const fn ending_letters(&self) -> &[Utf8Letter] {
        self.info.ending(unsafe { &*self.buf.as_ptr() })
    }
    pub const fn as_str(&self) -> &str {
        self.as_letters().as_str()
    }
    pub const fn stem(&self) -> &str {
        self.stem_letters().as_str()
    }
    pub const fn ending(&self) -> &str {
        self.ending_letters().as_str()
    }

    pub const fn borrow(&self) -> Word<'_> {
        Word { buf: unsafe { &*self.buf.as_ptr() }, info: self.info }
    }

    pub fn inflect<F: FnOnce(&mut [Utf8Letter]) -> Word<'_>>(&mut self, f: F) {
        let word = f(&mut self.buf);
        self.info = word.info;
    }
    pub fn into_string(self) -> String {
        self.buf.into_string(self.info.len as _)
    }
}

impl<'a> Word<'a> {
    pub(crate) const fn new(slice: &'a [Utf8Letter], stem_len: usize) -> Self {
        Self {
            buf: unsafe { &*slice.as_ptr() },
            info: WordInfo { len: slice.len() as _, stem_len: stem_len as _, insert_stress_pos: 0 },
        }
    }

    pub fn is_empty(&self) -> bool {
        self.info.len == 0
    }
    pub const fn as_letters(&self) -> &'a [Utf8Letter] {
        self.info.entire(self.buf)
    }
    pub const fn stem_letters(&self) -> &'a [Utf8Letter] {
        self.info.stem(self.buf)
    }
    pub const fn ending_letters(&self) -> &'a [Utf8Letter] {
        self.info.ending(self.buf)
    }
    pub const fn as_str(&self) -> &'a str {
        self.as_letters().as_str()
    }
    pub const fn stem(&self) -> &'a str {
        self.stem_letters().as_str()
    }
    pub const fn ending(&self) -> &'a str {
        self.ending_letters().as_str()
    }

    pub fn to_owned(&self) -> WordBuf {
        WordBuf { buf: self.as_letters().into(), info: self.info }
    }
}

// Implement some basic traits for WordBuf manually, since they can't be auto-derived
impl Clone for WordBuf {
    fn clone(&self) -> Self {
        Self { buf: self.as_letters().into(), info: self.info }
    }
}
impl PartialEq for WordBuf {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info && self.as_letters() == other.as_letters()
    }
}
impl Eq for WordBuf {}
impl std::hash::Hash for WordBuf {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.info.hash(state);
        self.as_letters().hash(state);
    }
}

// Implement some basic traits for Word manually, since they can't be auto-derived
impl const Default for Word<'_> {
    fn default() -> Self {
        Self { buf: unsafe { &*[].as_ptr() }, info: WordInfo::default() }
    }
}
impl const PartialEq for Word<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info && self.as_letters() == other.as_letters()
    }
}
impl Eq for Word<'_> {}
impl std::hash::Hash for Word<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.info.hash(state);
        self.as_letters().hash(state);
    }
}

// TODO: refactor to pass stress_pos
impl<'a> From<InflectionBuf<'a>> for Word<'a> {
    fn from(value: InflectionBuf<'a>) -> Self {
        let stem_len = value.stem_len / 2;
        Self::new(value.finish(), stem_len)
    }
}
