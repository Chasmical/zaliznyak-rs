/// A UTF-8-encoded lowercase cyrillic letter.
///
/// All lowercase cyrillic letters are 2 bytes wide in UTF-8, so a `[Utf8Letter]` can be safely
/// and cheaply cast to an equivalent `str`. However, not every lowercase cyrillic `str` can be
/// cast to an equivalent `[Utf8Letter]`, due to `str` having an alignment of 1, and `Utf8Letter`
/// requiring an alignment of 2.
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
#[rustfmt::skip]
#[repr(u16)]
pub enum Utf8Letter {
    #[doc(hidden)] А = d('а'), #[doc(hidden)] Б = d('б'),
    #[doc(hidden)] В = d('в'), #[doc(hidden)] Г = d('г'),
    #[doc(hidden)] Д = d('д'), #[doc(hidden)] Е = d('е'),
    #[doc(hidden)] Ж = d('ж'), #[doc(hidden)] З = d('з'),
    #[doc(hidden)] И = d('и'), #[doc(hidden)] Й = d('й'),
    #[doc(hidden)] К = d('к'), #[doc(hidden)] Л = d('л'),
    #[doc(hidden)] М = d('м'), #[doc(hidden)] Н = d('н'),
    #[doc(hidden)] О = d('о'), #[doc(hidden)] П = d('п'),
    #[doc(hidden)] Р = d('р'), #[doc(hidden)] С = d('с'),
    #[doc(hidden)] Т = d('т'), #[doc(hidden)] У = d('у'),
    #[doc(hidden)] Ф = d('ф'), #[doc(hidden)] Х = d('х'),
    #[doc(hidden)] Ц = d('ц'), #[doc(hidden)] Ч = d('ч'),
    #[doc(hidden)] Ш = d('ш'), #[doc(hidden)] Щ = d('щ'),
    #[doc(hidden)] Ъ = d('ъ'), #[doc(hidden)] Ы = d('ы'),
    #[doc(hidden)] Ь = d('ь'), #[doc(hidden)] Э = d('э'),
    #[doc(hidden)] Ю = d('ю'), #[doc(hidden)] Я = d('я'),
    #[doc(hidden)] Ё = d('ё'),
}

// TODO: Replace transmute with from_ne_bytes, when LSP/IDE fixes constants in pop-up window
#[allow(unnecessary_transmutes, reason = "u16::from_ne_bytes breaks constants in pop-up window")]
const fn d(ch: char) -> u16 {
    assert!(matches!(ch, 'а'..='я' | 'ё'));
    // SAFETY: This function is only used by the enum above, for 'а'..='я' | 'ё' only.
    unsafe { std::mem::transmute(encode_utf8_2(ch as u16)) }
}

const fn encode_utf8_2(codepoint: u16) -> [u8; 2] {
    [(codepoint >> 6 & 0x1F) as u8 | 0xC0, (codepoint & 0x3F) as u8 | 0x80]
}
const fn decode_utf8_2(utf8: [u8; 2]) -> u16 {
    (((utf8[0] & 0x1F) as u16) << 6) | ((utf8[1] & 0x3F) as u16)
}
const fn is_lowercase_cyrillic(utf8: [u8; 2]) -> bool {
    //                   [А..=П]       |        [Р..=Я | Ё]
    matches!(utf8, [0xD0, 0xB0..=0xBF] | [0xD1, 0x80..=0x8F | 0x91])
}

impl Utf8Letter {
    /// Constructs a `Utf8Letter` from UTF-8 bytes. Returns `None` if the UTF-8 bytes do not encode
    /// a valid lowercase cyrillic letter.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::Utf8Letter;
    ///
    /// assert_eq!(Utf8Letter::from_utf8([0xD0, 0xB0]), Some(Utf8Letter::А));
    /// assert_eq!(Utf8Letter::from_utf8([0xD0, 0xB6]), Some(Utf8Letter::Ж));
    /// assert_eq!(Utf8Letter::from_utf8([0xD1, 0x8E]), Some(Utf8Letter::Ю));
    ///
    /// assert_eq!(Utf8Letter::from_utf8([0xD1, 0x90]), None); // ѐ (U+0450 Ie With Grave)
    /// assert_eq!(Utf8Letter::from_utf8([0xC2, 0xB0]), None); // ° (U+00B0 Degree Sign)
    /// ```
    #[must_use]
    pub const fn from_utf8(utf8: [u8; 2]) -> Option<Self> {
        if is_lowercase_cyrillic(utf8) {
            // SAFETY: Just checked that the value is valid.
            Some(unsafe { Self::from_utf8_unchecked(utf8) })
        } else {
            None
        }
    }
    /// Constructs a `Utf8Letter` from UTF-8 bytes, without checking if it's a valid lowercase
    /// cyrillic letter.
    ///
    /// # Safety
    ///
    /// This function is unsafe, as it may construct invalid `Utf8Letter` values.
    ///
    /// For a safe version of this function, see the [`from_utf8`][Self::from_utf8] function.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::Utf8Letter;
    ///
    /// assert_eq!(unsafe { Utf8Letter::from_utf8_unchecked([0xD0, 0xB0]) }, Utf8Letter::А);
    /// assert_eq!(unsafe { Utf8Letter::from_utf8_unchecked([0xD0, 0xB6]) }, Utf8Letter::Ж);
    /// assert_eq!(unsafe { Utf8Letter::from_utf8_unchecked([0xD1, 0x8E]) }, Utf8Letter::Ю);
    /// ```
    #[must_use]
    pub const unsafe fn from_utf8_unchecked(utf8: [u8; 2]) -> Self {
        debug_assert!(is_lowercase_cyrillic(utf8));
        // SAFETY: The caller must uphold the safety contract.
        unsafe { std::mem::transmute(utf8) }
    }

    /// Constructs a `Utf8Letter` from a [`char`]. Returns `None` if it's not a valid lowercase
    /// cyrillic letter.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::Utf8Letter;
    ///
    /// assert_eq!(Utf8Letter::from_char('а'), Some(Utf8Letter::А));
    /// assert_eq!(Utf8Letter::from_char('п'), Some(Utf8Letter::П));
    /// assert_eq!(Utf8Letter::from_char('з'), Some(Utf8Letter::З));
    ///
    /// assert_eq!(Utf8Letter::from_char('ѐ'), None);
    /// assert_eq!(Utf8Letter::from_char('°'), None);
    /// ```
    #[must_use]
    pub const fn from_char(ch: char) -> Option<Self> {
        if matches!(ch, 'а'..='я' | 'ё') {
            // SAFETY: Just checked that the value is valid.
            Some(unsafe { Self::from_char_unchecked(ch) })
        } else {
            None
        }
    }
    /// Constructs a `Utf8Letter` from a [`char`], without checking if it's a valid lowercase
    /// cyrillic letter.
    ///
    /// # Safety
    ///
    /// This function is unsafe, as it may construct invalid `Utf8Letter` values.
    ///
    /// For a safe version of this function, see the [`from_char`][Self::from_char] function.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::Utf8Letter;
    ///
    /// assert_eq!(unsafe { Utf8Letter::from_char_unchecked('а') }, Utf8Letter::А);
    /// assert_eq!(unsafe { Utf8Letter::from_char_unchecked('п') }, Utf8Letter::П);
    /// assert_eq!(unsafe { Utf8Letter::from_char_unchecked('з') }, Utf8Letter::З);
    /// ```
    #[must_use]
    pub const unsafe fn from_char_unchecked(ch: char) -> Self {
        debug_assert!(matches!(ch, 'а'..='я' | 'ё'));
        // SAFETY: The caller must uphold the safety contract.
        unsafe { Self::from_utf8_unchecked(encode_utf8_2(ch as u16)) }
    }

    /// Returns this letter's UTF-8 bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::Utf8Letter;
    ///
    /// assert_eq!(Utf8Letter::И.to_utf8(), [0xD0, 0xB8]);
    /// assert_eq!(Utf8Letter::Ф.to_utf8(), [0xD1, 0x84]);
    /// assert_eq!(Utf8Letter::Ё.to_utf8(), [0xD1, 0x91]);
    /// ```
    #[must_use]
    pub const fn to_utf8(self) -> [u8; 2] {
        // SAFETY: Utf8Letter is always valid UTF-8.
        unsafe { std::mem::transmute(self) }
    }
    /// Returns a reference to this letter's UTF-8 bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::Utf8Letter;
    ///
    /// assert_eq!(Utf8Letter::И.as_utf8(), &[0xD0, 0xB8]);
    /// assert_eq!(Utf8Letter::Ф.as_utf8(), &[0xD1, 0x84]);
    /// assert_eq!(Utf8Letter::Ё.as_utf8(), &[0xD1, 0x91]);
    /// ```
    #[must_use]
    pub const fn as_utf8(&self) -> &[u8; 2] {
        // SAFETY: Utf8Letter is always valid UTF-8.
        unsafe { std::mem::transmute(self) }
    }
    /// Returns a reference to this letter's UTF-8 bytes as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::Utf8Letter;
    ///
    /// assert_eq!(Utf8Letter::И.as_str(), "и");
    /// assert_eq!(Utf8Letter::Ф.as_str(), "ф");
    /// assert_eq!(Utf8Letter::Ё.as_str(), "ё");
    /// ```
    #[must_use]
    pub const fn as_str(&self) -> &str {
        // SAFETY: Utf8Letter is always valid UTF-8.
        unsafe { str::from_utf8_unchecked(self.as_utf8()) }
    }

    /// Returns this letter's scalar [`char`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::Utf8Letter;
    ///
    /// assert_eq!(Utf8Letter::И.to_char(), 'и');
    /// assert_eq!(Utf8Letter::Ф.to_char(), 'ф');
    /// assert_eq!(Utf8Letter::Ё.to_char(), 'ё');
    /// ```
    #[must_use]
    pub const fn to_char(self) -> char {
        // SAFETY: Utf8Letter can always be safely decoded to a scalar Unicode value.
        unsafe { char::from_u32_unchecked(decode_utf8_2(self.to_utf8()) as u32) }
    }

    #[must_use]
    const fn last_byte(self) -> LetterLastByte {
        // SAFETY: LetterLastByte covers every possible last byte value.
        unsafe { std::mem::transmute(self.to_utf8()[1]) }
    }

    /// Returns `true` if this letter is a vowel (one of `аеиоуыэюяё`).
    #[must_use]
    pub const fn is_vowel(self) -> bool {
        use LetterLastByte::*;
        matches!(self.last_byte(), А | Е | И | О | У | Ы | Э | Ю | Я | Ё)
    }
    /// Returns `true` if this letter is a hissing sibilant (one of `жчшщ`).
    #[must_use]
    pub const fn is_hissing(self) -> bool {
        use LetterLastByte::*;
        matches!(self.last_byte(), Ж | Ч | Ш | Щ)
    }
    /// Returns `true` if this letter is a sibilant consonant (one of `жцчшщ`).
    #[must_use]
    pub const fn is_sibilant(self) -> bool {
        use LetterLastByte::*;
        matches!(self.last_byte(), Ж | Ц | Ч | Ш | Щ)
    }
    /// Returns `true` if this letter is a non-sibilant consonant (one of `бвгдзйклмнпрстфх`).
    #[must_use]
    pub const fn is_non_sibilant_consonant(self) -> bool {
        use LetterLastByte::*;
        matches!(self.last_byte(), Б | В | Г | Д | З | Й | К | Л | М | Н | П | Р | С | Т | Ф | Х)
    }
    /// Returns `true` if this letter is a consonant (one of `бвгджзйклмнпрстфхцчшщ`).
    #[must_use]
    #[rustfmt::skip]
    pub const fn is_consonant(self) -> bool {
        use LetterLastByte::*;
        matches!(self.last_byte(), Б | В | Г | Д | Ж | З | Й | К | Л | М | Н | П | Р | С | Т | Ф | Х | Ц | Ч | Ш | Щ)
    }

    /// Returns `true` if this letter, when being the last letter in the word with noun-type
    /// declension, is excluded from the word's stem (one of `аеийоуыьэюяё`).
    #[must_use]
    pub(crate) const fn is_stem_trim_letter(self) -> bool {
        use LetterLastByte::*;
        matches!(self.last_byte(), А | Е | И | Й | О | У | Ы | Ь | Э | Ю | Я | Ё)
    }
}

impl const AsRef<str> for Utf8Letter {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl const AsRef<[u8]> for Utf8Letter {
    fn as_ref(&self) -> &[u8] {
        self.as_utf8()
    }
}

// All the lowercase cyrillic letters' last UTF-8 bytes are distinct, meaning it's possible to
// match letters using the last byte only, making mass matches significantly more performant.
// TODO: This should be exposed in more places. It'd be very useful in parsing.
#[allow(dead_code)]
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
#[rustfmt::skip]
#[repr(u8)]
enum LetterLastByte {
    // [0xD0, 0xB0..=0xBF]
    А = 0xB0, Б, В, Г, Д, Е, Ж, З, И, Й, К, Л, М, Н, О, П,
    // [0xD1, 0x80..=0x8F]
    Р = 0x80, С, Т, У, Ф, Х, Ц, Ч, Ш, Щ, Ъ, Ы, Ь, Э, Ю, Я,
    // [0xD1, 0x91]
    Ё = 0x91,
}

mod private {
    pub trait Sealed {}
}

/// Provides [`as_str`][Utf8LetterSlice::as_str] method for the `[Utf8Letter]` slice.
pub const trait Utf8LetterSlice: private::Sealed {
    /// Casts this `[Utf8Letter]` slice to a `str` slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::{Utf8Letter::*, Utf8LetterSlice};
    ///
    /// let word = vec![Л, О, Ж, К, А];
    ///
    /// assert_eq!(word.as_str(), "ложка");
    /// assert_eq!(word[..3].as_str(), "лож");
    /// assert_eq!(word[3..].as_str(), "ка");
    /// ```
    #[must_use]
    fn as_str(&self) -> &str;
}

impl private::Sealed for [Utf8Letter] {}
impl const Utf8LetterSlice for [Utf8Letter] {
    fn as_str(&self) -> &str {
        // SAFETY: Utf8Letters represent 2-byte UTF-8 chunks, and can be safely cast to UTF-8.
        unsafe { std::str::from_raw_parts(self.as_ptr().cast(), self.len() * 2) }
    }
}

#[cfg(test)]
mod tests {
    use super::Utf8Letter::{self, *};

    #[rustfmt::skip]
    const VOWELS: [Utf8Letter; 10] = [А,Е,И,О,У,Ы,Э,Ю,Я,Ё];
    #[rustfmt::skip]
    const HISSING: [Utf8Letter; 4] = [Ж,Ч,Ш,Щ];
    #[rustfmt::skip]
    const SIBILANT: [Utf8Letter; 5] = [Ж,Ц,Ч,Ш,Щ];
    #[rustfmt::skip]
    const NON_SIBILANT_CONSONANTS: [Utf8Letter; 16] = [Б,В,Г,Д,З,Й,К,Л,М,Н,П,Р,С,Т,Ф,Х];
    #[rustfmt::skip]
    const CONSONANTS: [Utf8Letter; 21] = [Б,В,Г,Д,Ж,З,Й,К,Л,М,Н,П,Р,С,Т,Ф,Х,Ц,Ч,Ш,Щ];
    #[rustfmt::skip] #[allow(dead_code)]
    const ALL: [Utf8Letter; 33] = [А,Б,В,Г,Д,Е,Ж,З,И,Й,К,Л,М,Н,О,П,Р,С,Т,У,Ф,Х,Ц,Ч,Ш,Щ,Ъ,Ы,Ь,Э,Ю,Я,Ё];

    #[test]
    fn is_methods() {
        for x in VOWELS {
            assert!(x.is_vowel());
            assert!(!x.is_hissing());
            assert!(!x.is_sibilant());
            assert!(!x.is_non_sibilant_consonant());
            assert!(!x.is_consonant());
        }
        for x in HISSING {
            assert!(!x.is_vowel());
            assert!(x.is_hissing());
            assert!(x.is_sibilant());
            assert!(!x.is_non_sibilant_consonant());
            assert!(x.is_consonant());
        }
        for x in SIBILANT {
            assert!(!x.is_vowel());
            assert!(x.is_sibilant());
            assert!(!x.is_non_sibilant_consonant());
            assert!(x.is_consonant());
        }
        for x in NON_SIBILANT_CONSONANTS {
            assert!(!x.is_vowel());
            assert!(!x.is_hissing());
            assert!(!x.is_sibilant());
            assert!(x.is_non_sibilant_consonant());
            assert!(x.is_consonant());
        }
        for x in CONSONANTS {
            assert!(!x.is_vowel());
            assert!(x.is_consonant());
        }
    }
}
