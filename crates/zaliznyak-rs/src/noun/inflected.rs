use crate::{alphabet::Utf8Letter, util::StackBuf};

pub struct InflectedNounBuf {
    pub(crate) buf: StackBuf<Utf8Letter, 23>,
    pub(crate) len: usize,
    pub(crate) stem_len: usize,
}

pub struct InflectedNoun<'a> {
    pub(crate) buf: &'a mut [Utf8Letter],
    pub(crate) len: usize,
    pub(crate) stem_len: usize,
}

impl InflectedNounBuf {
    pub fn with_capacity(cap: usize) -> Self {
        let buf = StackBuf::with_capacity(cap);
        Self { buf, len: 0, stem_len: 0 }
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

    pub fn borrow<'a>(&'a mut self) -> InflectedNoun<'a> {
        InflectedNoun { buf: self.buf.as_mut_slice(), stem_len: self.stem_len, len: self.len }
    }
}
impl<'a> InflectedNoun<'a> {
    pub const fn as_str(&self) -> &str {
        Utf8Letter::cast(unsafe { self.buf.get_unchecked(..self.len) })
    }
    pub const fn stem(&self) -> &str {
        Utf8Letter::cast(unsafe { self.buf.get_unchecked(..self.stem_len) })
    }
    pub const fn ending(&self) -> &str {
        Utf8Letter::cast(unsafe { self.buf.get_unchecked(self.stem_len..self.len) })
    }

    pub fn to_owned(&self) -> InflectedNounBuf {
        InflectedNounBuf { buf: StackBuf::from(&*self.buf), stem_len: self.stem_len, len: self.len }
    }
}

impl const AsRef<str> for InflectedNounBuf {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl<'a> const AsRef<str> for InflectedNoun<'a> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> IntoIterator for &'a InflectedNounBuf {
    type Item = &'a str;
    type IntoIter = std::array::IntoIter<Self::Item, 1>;
    fn into_iter(self) -> Self::IntoIter {
        [self.as_str()].into_iter()
    }
}
impl<'a> IntoIterator for &'a InflectedNoun<'a> {
    type Item = &'a str;
    type IntoIter = std::array::IntoIter<Self::Item, 1>;
    fn into_iter(self) -> Self::IntoIter {
        [self.as_str()].into_iter()
    }
}
