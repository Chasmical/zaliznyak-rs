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
    pub const fn from_utf8(utf8: [u8; 2]) -> Option<Self> {
        if is_lowercase_russian_utf8(utf8) {
            // SAFETY: Just checked that the value is valid.
            Some(unsafe { Self::from_utf8_unchecked(utf8) })
        } else {
            None
        }
    }
    pub const unsafe fn from_utf8_unchecked(utf8: [u8; 2]) -> Self {
        debug_assert!(is_lowercase_russian_utf8(utf8));
        unsafe { std::mem::transmute(utf8) }
    }

    pub const fn from_char(ch: char) -> Option<Self> {
        if is_lowercase_russian_char(ch) {
            // SAFETY: Just checked that the value is valid.
            Some(unsafe { Self::from_char_unchecked(ch) })
        } else {
            None
        }
    }
    pub const unsafe fn from_char_unchecked(ch: char) -> Self {
        debug_assert!(is_lowercase_russian_char(ch));
        unsafe { std::mem::transmute(quick_encode_utf8(ch as u16)) }
    }

    pub const fn to_utf8(self) -> [u8; 2] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn as_utf8(&self) -> &[u8; 2] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_utf8()) }
    }

    pub const fn to_char(self) -> char {
        unsafe { char::from_u32_unchecked(quick_decode_utf8(self.to_utf8()) as u32) }
    }
    pub const fn to_byte(self) -> ByteLetter {
        unsafe { std::mem::transmute(self.to_utf8()[1]) }
    }

    pub const fn is_vowel(self) -> bool {
        self.to_byte().is_vowel()
    }
    pub const fn is_consonant(self) -> bool {
        self.to_byte().is_consonant()
    }
    pub const fn is_non_sibilant_consonant(self) -> bool {
        self.to_byte().is_non_sibilant_consonant()
    }
    pub const fn is_sibilant_consonant(self) -> bool {
        self.to_byte().is_sibilant_consonant()
    }
    pub const fn is_hissing_consonant(self) -> bool {
        self.to_byte().is_hissing_consonant()
    }
}

doc_hidden_variants! {
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
    pub const fn from_utf8(utf8: [u8; 2]) -> Option<Self> {
        Utf8Letter::from_utf8(utf8).map(Utf8Letter::to_byte)
    }
    pub const unsafe fn from_utf8_unchecked(utf8: [u8; 2]) -> Self {
        unsafe { Utf8Letter::from_utf8_unchecked(utf8) }.to_byte()
    }

    pub const fn from_char(ch: char) -> Option<Self> {
        Utf8Letter::from_char(ch).map(Utf8Letter::to_byte)
    }
    pub const unsafe fn from_char_unchecked(ch: char) -> Self {
        unsafe { Utf8Letter::from_char_unchecked(ch) }.to_byte()
    }

    pub const fn to_utf8(self) -> Utf8Letter {
        let first = if matches!(self as u8, 0xB0..=0xBF) { 0xD0 } else { 0xD1 };
        unsafe { Utf8Letter::from_utf8_unchecked([first, self as u8]) }
    }
    pub const fn to_char(self) -> char {
        const LOWER: u32 = 'а' as u32 - (ByteLetter::А as u32 & 0x3F);
        const UPPER: u32 = 'р' as u32 - (ByteLetter::Р as u32 & 0x3F);

        let base = if matches!(self as u8, 0xB0..=0xBF) { LOWER } else { UPPER };
        unsafe { char::from_u32_unchecked(base + (self as u32 & 0x3F)) }
    }

    pub const fn is_vowel(self) -> bool {
        use ByteLetter::*;
        matches!(self, А | Е | И | О | У | Ы | Э | Ю | Я | Ё)
    }
    #[rustfmt::skip]
    pub const fn is_consonant(self) -> bool {
        use ByteLetter::*;
        matches!(self, Б | В | Г | Д | Ж | З | Й | К | Л | М | Н | П | Р | С | Т | Ф | Х | Ц | Ч | Ш | Щ)
    }
    pub const fn is_non_sibilant_consonant(self) -> bool {
        use ByteLetter::*;
        matches!(self, Б | В | Г | Д | З | Й | К | Л | М | Н | П | Р | С | Т | Ф | Х)
    }
    pub const fn is_sibilant_consonant(self) -> bool {
        use ByteLetter::*;
        matches!(self, Ж | Ц | Ч | Ш | Щ)
    }
    pub const fn is_hissing_consonant(self) -> bool {
        use ByteLetter::*;
        matches!(self, Ж | Ч | Ш | Щ)
    }
}

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

pub const trait Utf8LetterSlice: private::Sealed {
    fn as_bytes(&self) -> &[u8];
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
