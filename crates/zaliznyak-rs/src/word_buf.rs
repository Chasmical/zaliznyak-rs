use crate::{
    alphabet::Utf8Letter,
    util::{InflectionBuf, StackBuf},
};

pub struct WordBuf {
    // Russian words very rarely exceed 23 letters
    pub(crate) buf: StackBuf<Utf8Letter, 23>,
    pub(crate) len: usize,
    pub(crate) stem_len: usize,
}

pub struct Word<'a> {
    pub(crate) buf: &'a mut [Utf8Letter],
    pub(crate) len: usize,
    pub(crate) stem_len: usize,
}

impl WordBuf {
    pub fn with_capacity(cap: usize) -> Self {
        let buf = StackBuf::with_capacity(cap);
        Self { buf, len: 0, stem_len: 0 }
    }
    pub(crate) fn with_capacity_for(stem: &str) -> Self {
        Self::with_capacity(InflectionBuf::max_char_len_for_noun(stem.len()))
    }

    pub const fn as_str(&self) -> &str {
        Utf8Letter::cast(unsafe { self.buf.get_unchecked(..self.len) })
    }
    pub const fn stem(&self) -> &str {
        Utf8Letter::cast(unsafe { self.buf.get_unchecked(..self.stem_len) })
    }
    pub const fn ending(&self) -> &str {
        Utf8Letter::cast(unsafe { self.buf.get_unchecked(self.stem_len..self.len) })
    }

    pub const fn borrow<'a>(&'a mut self) -> Word<'a> {
        Word { buf: self.buf.as_mut_slice(), stem_len: self.stem_len, len: self.len }
    }
    pub fn into_string(self) -> String {
        self.buf.into_string(self.len)
    }

    pub(crate) fn with_buf<'a>(
        mut self,
        inflect: impl FnOnce(&'a mut [Utf8Letter]) -> Word<'a>,
    ) -> Self {
        let dst = unsafe { std::mem::transmute(self.buf.as_mut_slice()) };
        let word = inflect(dst);
        self.stem_len = word.stem_len;
        self.len = word.len;
        self
    }
    pub(crate) fn with_buf_opt<'a>(
        mut self,
        inflect: impl FnOnce(&'a mut [Utf8Letter]) -> Option<Word<'a>>,
    ) -> Option<Self> {
        let dst = unsafe { std::mem::transmute(self.buf.as_mut_slice()) };
        if let Some(word) = inflect(dst) {
            self.stem_len = word.stem_len;
            self.len = word.len;
            Some(self)
        } else {
            None
        }
    }
}

impl<'a> Word<'a> {
    pub const fn as_str(&self) -> &str {
        Utf8Letter::cast(unsafe { self.buf.get_unchecked(..self.len) })
    }
    pub const fn stem(&self) -> &str {
        Utf8Letter::cast(unsafe { self.buf.get_unchecked(..self.stem_len) })
    }
    pub const fn ending(&self) -> &str {
        Utf8Letter::cast(unsafe { self.buf.get_unchecked(self.stem_len..self.len) })
    }

    pub fn to_owned(&self) -> WordBuf {
        WordBuf { buf: StackBuf::from(self.buf), stem_len: self.stem_len, len: self.len }
    }
    pub fn to_string(&self) -> String {
        Utf8Letter::cast(self.buf).to_owned()
    }
}

impl const AsRef<str> for WordBuf {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl<'a> const AsRef<str> for Word<'a> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> IntoIterator for &'a WordBuf {
    type Item = &'a str;
    type IntoIter = std::array::IntoIter<Self::Item, 1>;
    fn into_iter(self) -> Self::IntoIter {
        [self.as_str()].into_iter()
    }
}
impl<'a> IntoIterator for &'a Word<'a> {
    type Item = &'a str;
    type IntoIter = std::array::IntoIter<Self::Item, 1>;
    fn into_iter(self) -> Self::IntoIter {
        [self.as_str()].into_iter()
    }
}

impl std::fmt::Display for WordBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl<'a> std::fmt::Display for Word<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a> const From<InflectionBuf<'a>> for Word<'a> {
    fn from(value: InflectionBuf<'a>) -> Self {
        Self { len: value.len / 2, stem_len: value.stem_len / 2, buf: value.finish() }
    }
}
