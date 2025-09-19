use crate::{
    util::{InflectionBuf, StackVec},
    word::{Utf8Letter, Utf8LetterSlice},
};

#[derive(Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub(super) struct WordInfo {
    pub(super) stem_len: usize,
    pub(super) insert_stress_pos: usize,
}

#[derive(Clone, Eq, Hash)]
#[derive_const(Default, PartialEq)]
pub struct WordBuf {
    // Declinable parts of Russian words very rarely exceed 15 letters
    pub(super) buf: StackVec<Utf8Letter, 15>,
    pub(super) info: WordInfo,
}

#[derive(Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub struct Word<'a> {
    pub(super) buf: &'a [Utf8Letter],
    pub(super) info: WordInfo,
}

impl WordBuf {
    pub(crate) fn with_capacity_for(stem: &str) -> Self {
        Self::with_capacity(InflectionBuf::max_char_len_for_noun(stem.len()))
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self { buf: StackVec::with_capacity(cap), info: Default::default() }
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    pub const fn as_letters(&self) -> &[Utf8Letter] {
        &self.buf
    }
    pub const fn stem_letters(&self) -> &[Utf8Letter] {
        unsafe { self.buf.get_unchecked(..self.info.stem_len) }
    }
    pub const fn ending_letters(&self) -> &[Utf8Letter] {
        unsafe { self.buf.get_unchecked(self.info.stem_len..) }
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
        Word { buf: &self.buf, info: self.info }
    }

    pub fn inflect<F: FnOnce(&mut [Utf8Letter]) -> Word<'_>>(&mut self, f: F) {
        let dst = unsafe { self.buf.slice_full_capacity_mut().assume_init_mut() };
        let word = f(dst);

        self.info = word.info;
        let len = word.buf.len();
        unsafe { self.buf.set_len(len) };
    }
    pub fn into_string(self) -> String {
        self.buf.into_string()
    }
}

impl<'a> Word<'a> {
    // TODO: remake the constructor
    pub(crate) const fn new(buf: &'a [Utf8Letter], stem_len: usize) -> Self {
        Self { buf, info: WordInfo { stem_len, insert_stress_pos: 0 } }
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    pub const fn as_letters(&self) -> &'a [Utf8Letter] {
        self.buf
    }
    pub const fn stem_letters(&self) -> &'a [Utf8Letter] {
        unsafe { self.buf.get_unchecked(..self.info.stem_len) }
    }
    pub const fn ending_letters(&self) -> &'a [Utf8Letter] {
        unsafe { self.buf.get_unchecked(self.info.stem_len..) }
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
        WordBuf { buf: self.buf.into(), info: self.info }
    }
}

// TODO: refactor to pass stress_pos
impl<'a> From<InflectionBuf<'a>> for Word<'a> {
    fn from(value: InflectionBuf<'a>) -> Self {
        let stem_len = value.stem_len / 2;
        Self::new(value.finish(), stem_len)
    }
}
