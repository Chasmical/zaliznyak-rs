#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
#[rustfmt::skip]
pub enum Letter {
    А, Б, В, Г, Д, Е, Ж, З, И, Й, К, Л, М, Н, О, П,
    Р, С, Т, У, Ф, Х, Ц, Ч, Ш, Щ, Ъ, Ы, Ь, Э, Ю, Я,
    Ё = 33,
}

const fn encode_utf8_2(codepoint: u16) -> [u8; 2] {
    [(codepoint >> 6 & 0x1F) as u8 | 0xC0, (codepoint & 0x3F) as u8 | 0x80]
}
const fn decode_utf8_2(utf8: [u8; 2]) -> u16 {
    (((utf8[0] & 0x1F) as u16) << 6) | ((utf8[1] & 0x3F) as u16)
}

impl Letter {
    pub const fn from_codepoint(codepoint: u16) -> Option<Letter> {
        const A: u16 = 'а' as u16;
        const YA: u16 = 'я' as u16;
        const YO: u16 = 'ё' as u16;
        match codepoint {
            A..=YA | YO => Some(unsafe { Self::from_codepoint_unchecked(codepoint) }),
            _ => None,
        }
    }
    pub const fn from_char(ch: char) -> Option<Letter> {
        match ch {
            'а'..='я' | 'ё' => Some(unsafe { Self::from_char_unchecked(ch) }),
            _ => None,
        }
    }
    pub const fn from_utf8(utf8: [u8; 2]) -> Option<Letter> {
        Self::from_codepoint(decode_utf8_2(utf8))
    }

    pub const unsafe fn from_codepoint_unchecked(codepoint: u16) -> Letter {
        unsafe { std::mem::transmute((codepoint - 'а' as u16) as u8) }
    }
    pub const unsafe fn from_char_unchecked(ch: char) -> Letter {
        unsafe { Self::from_codepoint_unchecked(ch as u16) }
    }
    pub const unsafe fn from_utf8_unchecked(utf8: [u8; 2]) -> Letter {
        unsafe { Self::from_codepoint_unchecked(decode_utf8_2(utf8)) }
    }

    pub const fn to_codepoint(self) -> u16 {
        self as u16 + 'а' as u16
    }
    pub const fn to_char(self) -> char {
        unsafe { char::from_u32_unchecked(self.to_codepoint() as u32) }
    }
    pub const fn to_utf8(self) -> [u8; 2] {
        encode_utf8_2(self.to_codepoint())
    }
    pub const fn to_utf8_letter(self) -> Utf8Letter {
        unsafe { Utf8Letter::from_utf8_unchecked(self.to_utf8()) }
    }

    pub const fn is_vowel(self) -> bool {
        use Letter::*;
        matches!(self, А | Е | И | О | У | Ы | Э | Ю | Я | Ё)
    }
    pub const fn is_hissing(self) -> bool {
        use Letter::*;
        matches!(self, Ж | Ч | Ш | Щ)
    }
    pub const fn is_sibilant(self) -> bool {
        use Letter::*;
        matches!(self, Ж | Ц | Ч | Ш | Щ)
    }
    pub const fn is_non_sibilant_consonant(self) -> bool {
        use Letter::*;
        matches!(self, Б | В | Г | Д | З | Й | К | Л | М | Н | П | Р | С | Т | Ф | Х)
    }
    #[rustfmt::skip]
    pub const fn is_consonant(self) -> bool {
        use Letter::*;
        matches!(self, Б | В | Г | Д | Ж | З | Й | К | Л | М | Н | П | Р | С | Т | Ф | Х | Ц | Ч | Ш | Щ)
    }
}

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
#[allow(unnecessary_transmutes)]
const fn d(ch: char) -> u16 {
    unsafe { std::mem::transmute(Letter::from_char(ch).unwrap().to_utf8()) }
}

impl Utf8Letter {
    pub const fn from_utf8(utf8: [u8; 2]) -> Option<Self> {
        match Letter::from_codepoint(decode_utf8_2(utf8)) {
            Some(_) => Some(unsafe { Self::from_utf8_unchecked(utf8) }),
            None => None,
        }
    }
    pub const fn from_codepoint(codepoint: u16) -> Option<Self> {
        Letter::from_codepoint(codepoint).map(Letter::to_utf8_letter)
    }
    pub const fn from_char(ch: char) -> Option<Self> {
        Letter::from_char(ch).map(Letter::to_utf8_letter)
    }

    pub const unsafe fn from_utf8_unchecked(utf8: [u8; 2]) -> Self {
        unsafe { std::mem::transmute(utf8) }
    }
    pub const unsafe fn from_codepoint_unchecked(codepoint: u16) -> Self {
        unsafe { Self::from_utf8_unchecked(encode_utf8_2(codepoint)) }
    }
    pub const unsafe fn from_char_unchecked(ch: char) -> Self {
        unsafe { Self::from_codepoint_unchecked(ch as u16) }
    }

    pub const fn as_utf8(&self) -> &[u8; 2] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_utf8()) }
    }

    pub const fn to_utf8(self) -> [u8; 2] {
        unsafe { std::mem::transmute(self) }
    }
    pub const fn to_codepoint(self) -> u16 {
        decode_utf8_2(self.to_utf8())
    }
    pub const fn to_char(self) -> char {
        unsafe { char::from_u32_unchecked(self.to_codepoint() as u32) }
    }
    pub const fn to_letter(self) -> Letter {
        unsafe { Letter::from_codepoint_unchecked(self.to_codepoint()) }
    }

    pub const fn cast(letters: &[Utf8Letter]) -> &str {
        unsafe {
            let slice = std::slice::from_raw_parts(letters.as_ptr().cast(), letters.len() * 2);
            str::from_utf8_unchecked(slice)
        }
    }
    pub const fn cast_mut(letters: &mut [Utf8Letter]) -> &mut str {
        unsafe {
            let slice =
                std::slice::from_raw_parts_mut(letters.as_mut_ptr().cast(), letters.len() * 2);
            str::from_utf8_unchecked_mut(slice)
        }
    }

    pub const fn is_vowel(self) -> bool {
        self.to_letter().is_vowel()
    }
    pub const fn is_hissing(self) -> bool {
        self.to_letter().is_hissing()
    }
    pub const fn is_sibilant(self) -> bool {
        self.to_letter().is_sibilant()
    }
    pub const fn is_non_sibilant_consonant(self) -> bool {
        self.to_letter().is_non_sibilant_consonant()
    }
    pub const fn is_consonant(self) -> bool {
        self.to_letter().is_consonant()
    }
}

#[allow(dead_code)]
pub(crate) mod utf8 {
    use super::Utf8Letter;

    macro_rules! define_consts {
        ($($letter:ident)+) => ($(
            pub const $letter: [u8; 2] = Utf8Letter::$letter.to_utf8();
        )+);
    }
    define_consts! { А Б В Г Д Е Ё Ж З И Й К Л М Н О П Р С Т У Ф Х Ц Ч Ш Щ Ъ Ы Ь Э Ю Я }
}
