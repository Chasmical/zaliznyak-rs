// A macro to mark all of the enum's variants with `#[doc(hidden)]`.
macro_rules! doc_hidden_variants {
    (
        $(#[$outer:meta])* $vis:vis enum $T:ident {
            $( $(#[$inner:meta])* $var:ident $(= $val:expr)? ),* $(,)?
        }
    ) => (
        $(#[$outer])* $vis enum $T {
            $( $(#[$inner])* #[doc(hidden)] $var $(= $val)? ),*
        }
    );
}

doc_hidden_variants! {
    /// A UTF-8-encoded lowercase Russian letter.
    ///
    /// All lowercase Russian letters are 2 bytes wide in UTF-8, so a `[Utf8Letter]` can be safely
    /// and cheaply cast to an equivalent `str`. However, not every lowercase Russian `str` can be
    /// cast to an equivalent `[Utf8Letter]`, due to `str` having an alignment of 1, and `Utf8Letter`
    /// requiring an alignment of 2.
    #[derive(Debug, Copy, Eq, Hash)]
    #[derive_const(Clone, PartialEq)]
    #[rustfmt::skip]
    #[repr(u16)]
    pub enum Utf8Letter {
        А = d('а'), Б = d('б'), В = d('в'), Г = d('г'), Д = d('д'), Е = d('е'), Ж = d('ж'), З = d('з'),
        И = d('и'), Й = d('й'), К = d('к'), Л = d('л'), М = d('м'), Н = d('н'), О = d('о'), П = d('п'),
        Р = d('р'), С = d('с'), Т = d('т'), У = d('у'), Ф = d('ф'), Х = d('х'), Ц = d('ц'), Ч = d('ч'),
        Ш = d('ш'), Щ = d('щ'), Ъ = d('ъ'), Ы = d('ы'), Ь = d('ь'), Э = d('э'), Ю = d('ю'), Я = d('я'),
        Ё = d('ё'),
    }
}

#[allow(unnecessary_transmutes, reason = "TODO: from_ne_bytes breaks values in LSP/IDE popup")]
const fn d(ch: char) -> u16 {
    assert!(is_lowercase_russian_char(ch));
    // u16::from_ne_bytes(quick_encode(ch as u16))
    unsafe { std::mem::transmute(quick_encode_utf8(ch as u16)) }
}

// Specialized encode/decode fns for 2-byte UTF-8 codepoints.
const fn quick_encode_utf8(codepoint: u16) -> [u8; 2] {
    [(codepoint >> 6) as u8 | 0xC0, (codepoint & 0x3F) as u8 | 0x80]
}
const fn quick_decode_utf8(utf8: [u8; 2]) -> u16 {
    (((utf8[0] & 0x1F) as u16) << 6) | ((utf8[1] & 0x3F) as u16)
}

// Some helper functions to validate inputs
const fn is_lowercase_russian_char(ch: char) -> bool {
    matches!(ch, 'а'..='я' | 'ё')
}
const fn is_lowercase_russian_utf8(utf8: [u8; 2]) -> bool {
    //                   [А..=П]       |        [Р..=Я | Ё]
    matches!(utf8, [0xD0, 0xB0..=0xBF] | [0xD1, 0x80..=0x8F | 0x91])
}

impl Utf8Letter {
    /// Constructs a `Utf8Letter` from UTF-8 bytes. Returns `None` if the UTF-8 bytes do not encode
    /// a valid lowercase Russian letter.
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
        if is_lowercase_russian_utf8(utf8) {
            // SAFETY: Just checked that the value is valid.
            Some(unsafe { Self::from_utf8_unchecked(utf8) })
        } else {
            None
        }
    }
    /// Constructs a `Utf8Letter` from UTF-8 bytes, without checking if it's a valid lowercase
    /// Russian letter.
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
        debug_assert!(is_lowercase_russian_utf8(utf8));
        // SAFETY: The caller must uphold the safety contract.
        unsafe { std::mem::transmute(utf8) }
    }

    /// Constructs a `Utf8Letter` from a [`char`]. Returns `None` if it's not a valid lowercase
    /// Russian letter.
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
        if is_lowercase_russian_char(ch) {
            // SAFETY: Just checked that the value is valid.
            Some(unsafe { Self::from_char_unchecked(ch) })
        } else {
            None
        }
    }
    /// Constructs a `Utf8Letter` from a [`char`], without checking if it's a valid lowercase
    /// Russian letter.
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
        debug_assert!(is_lowercase_russian_char(ch));
        // SAFETY: The caller must uphold the safety contract.
        unsafe { std::mem::transmute(quick_encode_utf8(ch as u16)) }
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
        // SAFETY: Utf8Letter is always a valid UTF-8 sequence.
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
        // SAFETY: Utf8Letter is always a valid UTF-8 sequence.
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
        // SAFETY: Utf8Letter is always a valid UTF-8 sequence.
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
        self.to_byte().to_char()
    }
    /// Returns this letter's uniquely identifiable last byte, as [`ByteLetter`].
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::{ByteLetter, Utf8Letter};
    ///
    /// assert_eq!(Utf8Letter::И.to_byte(), ByteLetter::И);
    /// assert_eq!(Utf8Letter::Ф.to_byte(), ByteLetter::Ф);
    /// assert_eq!(Utf8Letter::Ё.to_byte(), ByteLetter::Ё);
    /// ```
    #[must_use]
    pub const fn to_byte(self) -> ByteLetter {
        // SAFETY: ByteLetter covers every possible last byte value.
        unsafe { std::mem::transmute(self.to_utf8()[1]) }
    }

    /// Returns `true` if this letter is a vowel (one of `аеиоуыэюяё`).
    #[must_use]
    pub const fn is_vowel(self) -> bool {
        self.to_byte().is_vowel()
    }
    /// Returns `true` if this letter is a consonant (one of `бвгджзйклмнпрстфхцчшщ`).
    #[must_use]
    pub const fn is_consonant(self) -> bool {
        self.to_byte().is_consonant()
    }
    /// Returns `true` if this letter is a non-sibilant consonant (one of `бвгдзйклмнпрстфх`).
    #[must_use]
    pub const fn is_non_sibilant_consonant(self) -> bool {
        self.to_byte().is_non_sibilant_consonant()
    }
    /// Returns `true` if this letter is a sibilant consonant (one of `жцчшщ`).
    #[must_use]
    pub const fn is_sibilant(self) -> bool {
        self.to_byte().is_sibilant()
    }
    /// Returns `true` if this letter is a hissing sibilant consonant (one of `жчшщ`).
    #[must_use]
    pub const fn is_hissing(self) -> bool {
        self.to_byte().is_hissing()
    }

    /// Returns `true` if this letter, when being the last letter in the word with noun-type
    /// declension, is excluded from the word's stem (one of `аеийоуыьэюяё`).
    #[must_use]
    pub(crate) const fn is_stem_trim_letter(self) -> bool {
        use ByteLetter::*;
        matches!(self.to_byte(), А | Е | И | Й | О | У | Ы | Ь | Э | Ю | Я | Ё)
    }
}

doc_hidden_variants! {
    /// Uniquely identifiable last byte of [`Utf8Letter`].
    ///
    /// All the lowercase Russian letters' last UTF-8 bytes are distinct, meaning it's possible to
    /// match letters using the last byte only, making mass matches significantly more performant
    /// (emitting the `bt` instruction, --- bitstring lookup).
    #[derive(Debug, Copy, Eq, Hash)]
    #[derive_const(Clone, PartialEq)]
    #[rustfmt::skip]
    #[repr(u8)]
    pub enum ByteLetter {
        // [0xD0, 0xB0..=0xBF]
        А = 0xB0, Б, В, Г, Д, Е, Ж, З, И, Й, К, Л, М, Н, О, П,
        // [0xD1, 0x80..=0x8F]
        Р = 0x80, С, Т, У, Ф, Х, Ц, Ч, Ш, Щ, Ъ, Ы, Ь, Э, Ю, Я,
        // [0xD1, 0x91]
        Ё = 0x91,
    }
}

impl ByteLetter {
    /// Constructs a `ByteLetter` from UTF-8 bytes. Returns `None` if the UTF-8 bytes do not encode
    /// a valid lowercase Russian letter.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::ByteLetter;
    ///
    /// assert_eq!(ByteLetter::from_utf8([0xD0, 0xB0]), Some(ByteLetter::А));
    /// assert_eq!(ByteLetter::from_utf8([0xD0, 0xB6]), Some(ByteLetter::Ж));
    /// assert_eq!(ByteLetter::from_utf8([0xD1, 0x8E]), Some(ByteLetter::Ю));
    ///
    /// assert_eq!(ByteLetter::from_utf8([0xD1, 0x90]), None); // ѐ (U+0450 Ie With Grave)
    /// assert_eq!(ByteLetter::from_utf8([0xC2, 0xB0]), None); // ° (U+00B0 Degree Sign)
    /// ```
    #[must_use]
    pub const fn from_utf8(utf8: [u8; 2]) -> Option<Self> {
        Utf8Letter::from_utf8(utf8).map(Utf8Letter::to_byte)
    }
    /// Constructs a `ByteLetter` from UTF-8 bytes, without checking if it's a valid lowercase
    /// Russian letter.
    ///
    /// # Safety
    ///
    /// This function is unsafe, as it may construct invalid `ByteLetter` values.
    ///
    /// For a safe version of this function, see the [`from_utf8`][Self::from_utf8] function.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::ByteLetter;
    ///
    /// assert_eq!(unsafe { ByteLetter::from_utf8_unchecked([0xD0, 0xB0]) }, ByteLetter::А);
    /// assert_eq!(unsafe { ByteLetter::from_utf8_unchecked([0xD0, 0xB6]) }, ByteLetter::Ж);
    /// assert_eq!(unsafe { ByteLetter::from_utf8_unchecked([0xD1, 0x8E]) }, ByteLetter::Ю);
    /// ```
    #[must_use]
    pub const unsafe fn from_utf8_unchecked(utf8: [u8; 2]) -> Self {
        // SAFETY: The caller must uphold the safety contract.
        unsafe { Utf8Letter::from_utf8_unchecked(utf8) }.to_byte()
    }

    /// Constructs a `ByteLetter` from a [`char`]. Returns `None` if it's not a valid lowercase
    /// Russian letter.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::ByteLetter;
    ///
    /// assert_eq!(ByteLetter::from_char('а'), Some(ByteLetter::А));
    /// assert_eq!(ByteLetter::from_char('п'), Some(ByteLetter::П));
    /// assert_eq!(ByteLetter::from_char('з'), Some(ByteLetter::З));
    ///
    /// assert_eq!(ByteLetter::from_char('ѐ'), None);
    /// assert_eq!(ByteLetter::from_char('°'), None);
    /// ```
    #[must_use]
    pub const fn from_char(ch: char) -> Option<Self> {
        Utf8Letter::from_char(ch).map(Utf8Letter::to_byte)
    }
    /// Constructs a `ByteLetter` from a [`char`], without checking if it's a valid lowercase
    /// Russian letter.
    ///
    /// # Safety
    ///
    /// This function is unsafe, as it may construct invalid `ByteLetter` values.
    ///
    /// For a safe version of this function, see the [`from_char`][Self::from_char] function.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::ByteLetter;
    ///
    /// assert_eq!(unsafe { ByteLetter::from_char_unchecked('а') }, ByteLetter::А);
    /// assert_eq!(unsafe { ByteLetter::from_char_unchecked('п') }, ByteLetter::П);
    /// assert_eq!(unsafe { ByteLetter::from_char_unchecked('з') }, ByteLetter::З);
    /// ```
    #[must_use]
    pub const unsafe fn from_char_unchecked(ch: char) -> Self {
        // SAFETY: The caller must uphold the safety contract.
        unsafe { Utf8Letter::from_char_unchecked(ch) }.to_byte()
    }

    /// Returns this letter's UTF-8 bytes, as [`Utf8Letter`].
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::{ByteLetter, Utf8Letter};
    ///
    /// assert_eq!(ByteLetter::И.to_utf8(), Utf8Letter::И);
    /// assert_eq!(ByteLetter::Ф.to_utf8(), Utf8Letter::Ф);
    /// assert_eq!(ByteLetter::Ё.to_utf8(), Utf8Letter::Ё);
    /// ```
    #[must_use]
    pub const fn to_utf8(self) -> Utf8Letter {
        let first = if matches!(self as u8, 0xB0..=0xBF) { 0xD0 } else { 0xD1 };
        unsafe { Utf8Letter::from_utf8_unchecked([first, self as u8]) }
    }
    /// Returns this letter's scalar [`char`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::ByteLetter;
    ///
    /// assert_eq!(ByteLetter::И.to_char(), 'и');
    /// assert_eq!(ByteLetter::Ф.to_char(), 'ф');
    /// assert_eq!(ByteLetter::Ё.to_char(), 'ё');
    /// ```
    #[must_use]
    pub const fn to_char(self) -> char {
        let offset = (self as u8 + 16) & 0x3F;
        unsafe { char::from_u32_unchecked('а' as u32 + offset as u32) }
    }

    /// Returns `true` if this letter is a vowel (one of `аеиоуыэюяё`).
    #[must_use]
    pub const fn is_vowel(self) -> bool {
        use ByteLetter::*;
        matches!(self, А | Е | И | О | У | Ы | Э | Ю | Я | Ё)
    }
    /// Returns `true` if this letter is a consonant (one of `бвгджзйклмнпрстфхцчшщ`).
    #[must_use]
    #[rustfmt::skip]
    pub const fn is_consonant(self) -> bool {
        use ByteLetter::*;
        matches!(self, Б | В | Г | Д | Ж | З | Й | К | Л | М | Н | П | Р | С | Т | Ф | Х | Ц | Ч | Ш | Щ)
    }
    /// Returns `true` if this letter is a non-sibilant consonant (one of `бвгдзйклмнпрстфх`).
    #[must_use]
    pub const fn is_non_sibilant_consonant(self) -> bool {
        use ByteLetter::*;
        matches!(self, Б | В | Г | Д | З | Й | К | Л | М | Н | П | Р | С | Т | Ф | Х)
    }
    /// Returns `true` if this letter is a sibilant consonant (one of `жцчшщ`).
    #[must_use]
    pub const fn is_sibilant(self) -> bool {
        use ByteLetter::*;
        matches!(self, Ж | Ц | Ч | Ш | Щ)
    }
    /// Returns `true` if this letter is a hissing sibilant consonant (one of `жчшщ`).
    #[must_use]
    pub const fn is_hissing(self) -> bool {
        use ByteLetter::*;
        matches!(self, Ж | Ч | Ш | Щ)
    }
}

// Some convenient conversion impls
impl const From<Utf8Letter> for ByteLetter {
    fn from(value: Utf8Letter) -> Self {
        value.to_byte()
    }
}
impl const From<ByteLetter> for Utf8Letter {
    fn from(value: ByteLetter) -> Self {
        value.to_utf8()
    }
}
impl const From<Utf8Letter> for char {
    fn from(value: Utf8Letter) -> Self {
        value.to_char()
    }
}
impl const From<ByteLetter> for char {
    fn from(value: ByteLetter) -> Self {
        value.to_char()
    }
}
impl const TryFrom<char> for Utf8Letter {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Self::from_char(value).ok_or(())
    }
}
impl const TryFrom<char> for ByteLetter {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Self::from_char(value).ok_or(())
    }
}

impl const AsRef<[u8]> for Utf8Letter {
    fn as_ref(&self) -> &[u8] {
        self.as_utf8()
    }
}
impl const AsRef<str> for Utf8Letter {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Utf8Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::fmt::Display for ByteLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.to_utf8().as_str().fmt(f)
    }
}

mod private {
    pub trait Sealed {}
}

/// Provides [`as_str`][Utf8LetterSlice::as_str] and [`as_bytes`][Utf8LetterSlice::as_bytes] methods
/// for the `[Utf8Letter]` slice.
pub const trait Utf8LetterSlice: private::Sealed {
    /// Casts this `[Utf8Letter]` slice to a `[u8]` slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::word::{Utf8Letter::*, Utf8LetterSlice};
    ///
    /// let word = vec![М, Ё, Д];
    ///
    /// assert_eq!(word.as_bytes(), &[0xD0, 0xBC, 0xD1, 0x91, 0xD0, 0xB4]);
    /// assert_eq!(word[..2].as_bytes(), &[0xD0, 0xBC, 0xD1, 0x91]);
    /// assert_eq!(word[2..].as_bytes(), &[0xD0, 0xB4]);
    /// ```
    #[must_use]
    fn as_bytes(&self) -> &[u8];
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
    fn as_bytes(&self) -> &[u8] {
        // SAFETY: Utf8Letters represent 2-byte UTF-8 chunks, and can be safely cast to UTF-8.
        unsafe { std::slice::from_raw_parts(self.as_ptr().cast(), self.len() * 2) }
    }
    fn as_str(&self) -> &str {
        // SAFETY: Utf8Letters represent 2-byte UTF-8 chunks, and can be safely cast to UTF-8.
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants() {
        assert_eq!(Utf8Letter::А.to_char(), 'а');
        assert_eq!(Utf8Letter::Б.to_char(), 'б');
        assert_eq!(Utf8Letter::Ш.to_char(), 'ш');
        assert_eq!(Utf8Letter::Я.to_char(), 'я');
        assert_eq!(Utf8Letter::Ё.to_char(), 'ё');

        assert_eq!(ByteLetter::А.to_char(), 'а');
        assert_eq!(ByteLetter::Б.to_char(), 'б');
        assert_eq!(ByteLetter::Ш.to_char(), 'ш');
        assert_eq!(ByteLetter::Я.to_char(), 'я');
        assert_eq!(ByteLetter::Ё.to_char(), 'ё');
    }

    #[test]
    fn encoding() {
        for ch in "абвгдеёжзийклмнопрстуфхцчшщъыьэюя".chars() {
            let mut utf8 = [0; 2];
            ch.encode_utf8(&mut utf8);

            let utf8_letter = Utf8Letter::from_char(ch).unwrap();
            assert_eq!(utf8_letter, Utf8Letter::from_utf8(utf8).unwrap());

            assert_eq!(utf8_letter.to_char(), ch);
            assert_eq!(utf8_letter.to_utf8(), utf8);
            assert_eq!(utf8_letter.as_str(), ch.to_string());

            let byte_letter = ByteLetter::from_char(ch).unwrap();
            assert_eq!(byte_letter, ByteLetter::from_utf8(utf8).unwrap());

            assert_eq!(byte_letter.to_char(), ch);
            assert_eq!(byte_letter.to_utf8(), utf8_letter);
        }
    }
}
