use crate::{
    util::{InflectionBuf, StackVec},
    word::{Utf8Letter, Utf8LetterSlice},
};

#[derive(Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub(crate) struct WordInfo {
    pub stem_len: usize,
    pub insert_stress_pos: usize,
}

// size_of::<WordBuf>()
//   = StackVec discriminant (2 bytes)
//   + 15 letters (30 bytes)
//   + len & info (3 usizes)
//   = 56 bytes (64x) or 44 bytes (32x)
const WORD_BUF_LETTERS: usize = 15;

/// A UTF-8-encoded lowercase cyrillic string.
///
/// # Examples
///
/// ```
/// use zaliznyak::word::WordBuf;
///
/// let buf: WordBuf = "сло́в-о".parse().unwrap();
///
/// assert_eq!(buf.as_str(), "слово");
/// assert_eq!(buf.stem(), "слов");
/// assert_eq!(buf.ending(), "о");
///
/// assert_eq!(format!("{}", buf), "сло́во");
/// assert_eq!(format!("{:?}", buf), "сло́в-о");
/// ```
#[derive(Clone, Eq, Hash)]
#[derive_const(Default, PartialEq)]
pub struct WordBuf {
    // Declinable parts of Russian words very rarely exceed 15 letters
    pub(super) buf: StackVec<Utf8Letter, WORD_BUF_LETTERS>,
    pub(super) info: WordInfo,
}

/// A UTF-8-encoded lowercase cyrillic string slice.
///
/// # Examples
///
/// ```
/// use zaliznyak::word::{Word, WordBuf};
///
/// let buf: WordBuf = "сло́в-о".parse().unwrap();
/// let word: Word = buf.borrow();
///
/// assert_eq!(word.as_str(), "слово");
/// assert_eq!(word.stem(), "слов");
/// assert_eq!(word.ending(), "о");
///
/// assert_eq!(format!("{}", word), "сло́во");
/// assert_eq!(format!("{:?}", word), "сло́в-о");
/// ```
#[derive(Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub struct Word<'a> {
    pub(super) buf: &'a [Utf8Letter],
    pub(super) info: WordInfo,
}

impl WordBuf {
    #[must_use]
    pub(crate) fn with_capacity_for(stem: &str) -> Self {
        Self::with_capacity(InflectionBuf::max_char_len_for_noun(stem.len()))
    }
    #[must_use]
    pub(crate) fn with_capacity(cap: usize) -> Self {
        Self { buf: StackVec::with_capacity(cap), info: Default::default() }
    }

    /// Returns `true` if this `WordBuf` is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    /// Returns the word as letters.
    #[must_use]
    pub const fn as_letters(&self) -> &[Utf8Letter] {
        &self.buf
    }
    /// Returns the word's stem as letters.
    #[must_use]
    pub const fn stem_letters(&self) -> &[Utf8Letter] {
        unsafe { self.buf.get_unchecked(..self.info.stem_len) }
    }
    /// Returns the word's ending as letters.
    #[must_use]
    pub const fn ending_letters(&self) -> &[Utf8Letter] {
        unsafe { self.buf.get_unchecked(self.info.stem_len..) }
    }
    /// Returns the word as a UTF-8-encoded string.
    #[must_use]
    pub const fn as_str(&self) -> &str {
        self.as_letters().as_str()
    }
    /// Returns the word's stem as a UTF-8-encoded string.
    #[must_use]
    pub const fn stem(&self) -> &str {
        self.stem_letters().as_str()
    }
    /// Returns the word's ending as a UTF-8-encoded string.
    #[must_use]
    pub const fn ending(&self) -> &str {
        self.ending_letters().as_str()
    }

    /// Returns a read-only [`Word`] slice of this `WordBuf`.
    #[must_use]
    pub const fn borrow(&self) -> Word<'_> {
        Word { buf: &self.buf, info: self.info }
    }
    /// Converts the word into a [`String`].
    #[must_use]
    pub fn into_string(self) -> String {
        self.buf.into_string()
    }

    pub(crate) const fn inflect<F: [const] FnOnce(&mut [Utf8Letter]) -> Word<'_>>(&mut self, f: F) {
        let dst = unsafe { self.buf.slice_full_capacity_mut().assume_init_mut() };
        let word = f(dst);

        self.info = word.info;
        let len = word.buf.len();
        unsafe { self.buf.set_len(len) };
    }
}

impl<'a> Word<'a> {
    #[must_use]
    pub(crate) const fn new(buf: &'a [Utf8Letter], info: WordInfo) -> Self {
        debug_assert!(info.insert_stress_pos <= buf.len());
        debug_assert!(info.stem_len <= buf.len());
        Self { buf, info }
    }

    /// Returns `true` if this `Word` is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    /// Returns the word as letters.
    #[must_use]
    pub const fn as_letters(&self) -> &'a [Utf8Letter] {
        self.buf
    }
    /// Returns the word's stem as letters.
    #[must_use]
    pub const fn stem_letters(&self) -> &'a [Utf8Letter] {
        unsafe { self.buf.get_unchecked(..self.info.stem_len) }
    }
    /// Returns the word's ending as letters.
    #[must_use]
    pub const fn ending_letters(&self) -> &'a [Utf8Letter] {
        unsafe { self.buf.get_unchecked(self.info.stem_len..) }
    }
    /// Returns the word as a UTF-8-encoded string.
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.as_letters().as_str()
    }
    /// Returns the word's stem as a UTF-8-encoded string.
    #[must_use]
    pub const fn stem(&self) -> &'a str {
        self.stem_letters().as_str()
    }
    /// Returns the word's ending as a UTF-8-encoded string.
    #[must_use]
    pub const fn ending(&self) -> &'a str {
        self.ending_letters().as_str()
    }

    /// Creates an owned [`WordBuf`] from this word slice.
    #[must_use]
    pub fn to_owned(&self) -> WordBuf {
        WordBuf { buf: self.buf.into(), info: self.info }
    }
}

// TODO: refactor to pass stress_pos
impl<'a> const From<InflectionBuf<'a>> for Word<'a> {
    fn from(value: InflectionBuf<'a>) -> Self {
        let stem_len = value.stem_len / 2;
        Self::new(value.finish(), WordInfo { stem_len, insert_stress_pos: 0 })
    }
}
