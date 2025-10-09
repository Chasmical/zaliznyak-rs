//! Lowercase cyrillic words and letters.
//!
//! # Words
//!
//! Inflected words in this library are represented using [`WordBuf`] (equivalent of [`String`])
//! and [`Word<'_>`] (equivalent of `&`[`str`]). They still represent UTF-8-encoded strings, but
//! can only contain lowercase cyrillic letters. The library relies on this contract to process
//! and inflect the words significantly faster, than if reading individual UTF-8 bytes. Inflected
//! words also contain some extra info, used by inflection: stem length and stress position.
//!
//! ```
//! use zaliznyak::word::{WordBuf, Word};
//!
//! let buf: WordBuf = "сло́в-о".parse().unwrap();
//!
//! assert_eq!(buf.as_str(), "слово");
//! assert_eq!(buf.stem(), "слов");
//! assert_eq!(buf.ending(), "о");
//!
//! assert_eq!(format!("{}", buf), "сло́во");
//! assert_eq!(format!("{:?}", buf), "сло́в-о");
//!
//! let word: Word = buf.borrow();
//! ```
//!
//! Short words (≤10 letters) are inflected on the stack, for performance. Words longer than that
//! are usually composite, consisting of multiple words joined together (sometimes with hyphens),
//! and are represented using...
//! // TODO
//!
//! # Letters
//!
//! Individual letters in words are represented using [`Utf8Letter`].
//!
//! ```
//! use zaliznyak::word::{Utf8Letter::*, Utf8LetterSlice, WordBuf};
//!
//! // The stress here is automatically inferred to be on 'ё'
//! let buf: WordBuf = "мёд-ом".parse().unwrap();
//!
//! assert_eq!(buf.as_letters(), [М, Ё, Д, О, М]);
//! assert_eq!(buf.stem_letters(), [М, Ё, Д]);
//! assert_eq!(buf.ending_letters(), [О, М]);
//!
//! assert_eq!(buf.as_letters()[0].is_consonant(), true);
//! assert_eq!(buf.as_letters()[1].is_vowel(), true);
//! assert_eq!(buf.as_letters()[2].as_str(), "д");
//! assert_eq!(buf.as_letters()[2..].as_str(), "дом");
//! ```
//!
//! # Parsing and formatting
//!
//! If present, the stress indicator in the parsed string must be in one of the following forms:
//! `о́` (U+0301 Combining Acute Accent), `о̀` (U+0300 Combining Grave Accent), or `о'` (ASCII
//! Apostrophe; for simple keyboard input).
//!
//! The stress indicator may be omitted from the parsed string, but only when it can be safely
//! inferred from the rest of the word; that is, either a) There's only one vowel in the word that
//! can receive stress, or b) The stress is on letter 'ё' which is always stressed in Russian words
//! (with the only exceptions being a few foreign surnames).
//!
//! The ending separator (`-` ASCII Hyphen-Minus) may be used to separate the stem from the ending.
//! If the ending separator is not present, then the entire word is assumed to be the stem.
//!
//! ```
//! use zaliznyak::word::{ParseWordError, WordBuf};
//!
//! let buf: WordBuf = "сло'в-о".parse().unwrap();
//! assert_eq!(format!("{:?}", buf), "сло́в-о");
//!
//! let buf: WordBuf = "порт".parse().unwrap();
//! assert_eq!(format!("{:?}", buf), "по́рт");
//!
//! let buf: WordBuf = "мёд-ом".parse().unwrap();
//! assert_eq!(format!("{:?}", buf), "мё́д-ом");
//!
//! let buf: WordBuf = "сёра̀".parse().unwrap();
//! assert_eq!(format!("{:?}", buf), "сёра́");
//!
//! assert_eq!("слов-о".parse::<WordBuf>(), Err(ParseWordError::NoStress));
//! ```

mod display;
mod from_str;
mod letter;

pub use display::*;
pub use from_str::*;
pub use letter::*;

use crate::util::{InflectionBuf, StackVec};

/// Max amount of letters that can be stored in [`WordBuf`] on the stack.
///
/// # Stack vs. Heap
///
/// Vec = ptr+len+cap. Length is required either way, so, excluding length, Vec's size is 2 usizes.
///
/// | Letters | bytes     | 64x      | 32x      |
/// |---------|-----------|----------|----------|
/// | 7       | 16=14+2   | 2  (+0)  | 4  (+2)  |
/// | 11      | 24=22+2   | 3  (+1)  | 6  (+4)  |
/// | 15      | 32=30+2   | 4  (+2)  | 8  (+6)  |
/// | 19      | 40=38+2   | 5  (+3)  | 10 (+8)  |
/// | 23      | 48=46+2   | 6  (+4)  | 12 (+10) |
/// | 27      | 56=54+2   | 7  (+5)  | 14 (+12) |
///
/// And here's a quick analysis of the words' stems in Zaliznyak's dictionary:
///
/// | Stem length | Words | Coverage |
/// |------------:|------:|---------:|
/// | ≤ 4 letters |  4821 |     7.2% |
/// | ≤ 5 letters | 10799 |    16.1% |
/// | ≤ 6 letters | 19861 |    29.7% |
/// | ≤ 7 letters | 29894 |    44.7% |
/// | ≤ 8 letters | 40136 |    60.0% |
/// | ≤ 9 letters | 48765 |    72.8% |
/// | ≤10 letters | 55304 |    82.6% |
/// | ≤14 letters | 65763 |    98.2% |
/// | ≤18 letters | 66910 |    99.9% |
/// | ≤22 letters | 66947 |   100.0% |
///
/// TODO: recalc the table later with a more accurate method (above was done with regexes).
///
/// Allocating a 15-letter stack buffer allows for on-stack declension of 10-letter stems, covering
/// 82.6% of all the words in Zaliznyak's dictionary. Considering that most words that are longer
/// than 10 letters are either extremely specific or rare, the 82.6% percentage is actually more
/// than enough for general-purpose declension.
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
    pub(super) stem_len: usize,
    pub(super) stress_at: usize,
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
    pub(super) stem_len: usize,
    pub(super) stress_at: usize,
}

impl WordBuf {
    #[must_use]
    pub(crate) fn with_capacity_for(stem: &str) -> Self {
        Self::with_capacity(InflectionBuf::max_char_len_for_noun(stem.len()))
    }
    #[must_use]
    pub(crate) fn with_capacity(cap: usize) -> Self {
        Self { buf: StackVec::with_capacity(cap), stem_len: 0, stress_at: 0 }
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
        // SAFETY: stem_len is always within valid range.
        unsafe { self.buf.get_unchecked(..self.stem_len) }
    }
    /// Returns the word's ending as letters.
    #[must_use]
    pub const fn ending_letters(&self) -> &[Utf8Letter] {
        // SAFETY: stem_len is always within valid range.
        unsafe { self.buf.get_unchecked(self.stem_len..) }
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
        Word { buf: &self.buf, stem_len: self.stem_len, stress_at: self.stress_at }
    }
    /// Converts the word into a [`String`].
    #[must_use]
    pub fn into_string(self) -> String {
        self.buf.into_string()
    }

    // TODO: mark unsafe, since it could potentially access uninit letters?
    pub(crate) const fn inflect<F: [const] FnOnce(&mut [Utf8Letter]) -> Word<'_>>(&mut self, f: F) {
        let dst = unsafe { self.buf.slice_full_capacity_mut().assume_init_mut() };
        let word = f(dst);

        self.stem_len = word.stem_len;
        self.stress_at = word.stress_at;
        let len = word.buf.len();
        unsafe { self.buf.set_len(len) };
    }
}

impl<'a> Word<'a> {
    #[must_use]
    pub(crate) const fn new(buf: &'a [Utf8Letter], stem_len: usize, stress_at: usize) -> Self {
        debug_assert!(stress_at <= buf.len());
        debug_assert!(stem_len <= buf.len());
        Self { buf, stem_len, stress_at }
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
        // SAFETY: stem_len is always within valid range.
        unsafe { self.buf.get_unchecked(..self.stem_len) }
    }
    /// Returns the word's ending as letters.
    #[must_use]
    pub const fn ending_letters(&self) -> &'a [Utf8Letter] {
        // SAFETY: stem_len is always within valid range.
        unsafe { self.buf.get_unchecked(self.stem_len..) }
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
        WordBuf { buf: self.buf.into(), stem_len: self.stem_len, stress_at: self.stress_at }
    }
}

impl const AsRef<[Utf8Letter]> for WordBuf {
    fn as_ref(&self) -> &[Utf8Letter] {
        self.as_letters()
    }
}
impl const AsRef<[Utf8Letter]> for Word<'_> {
    fn as_ref(&self) -> &[Utf8Letter] {
        self.as_letters()
    }
}

impl const AsRef<str> for WordBuf {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl const AsRef<str> for Word<'_> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// TODO: refactor to pass stress_pos
impl<'a> const From<InflectionBuf<'a>> for Word<'a> {
    fn from(value: InflectionBuf<'a>) -> Self {
        let stem_len = value.stem_len / 2;
        Self::new(value.finish(), stem_len, 0)
    }
}
