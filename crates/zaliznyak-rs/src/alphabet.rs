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
    unsafe { std::mem::transmute(encode_utf8_2(ch as u16)) }
}

const fn encode_utf8_2(codepoint: u16) -> [u8; 2] {
    [(codepoint >> 6 & 0x1F) as u8 | 0xC0, (codepoint & 0x3F) as u8 | 0x80]
}
const fn decode_utf8_2(utf8: [u8; 2]) -> u16 {
    (((utf8[0] & 0x1F) as u16) << 6) | ((utf8[1] & 0x3F) as u16)
}

impl Utf8Letter {
    pub const unsafe fn from_utf8_unchecked(utf8: [u8; 2]) -> Self {
        unsafe { std::mem::transmute(utf8) }
    }
    pub const fn from_utf8(utf8: [u8; 2]) -> Option<Self> {
        if utf8::is_defined(utf8) { Some(unsafe { Self::from_utf8_unchecked(utf8) }) } else { None }
    }

    pub const unsafe fn from_char_unchecked(ch: char) -> Self {
        unsafe { Self::from_utf8_unchecked(encode_utf8_2(ch as u16)) }
    }
    pub const fn from_char(ch: char) -> Option<Self> {
        if matches!(ch, 'а'..='я' | 'ё') {
            Some(unsafe { Self::from_char_unchecked(ch) })
        } else {
            None
        }
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
        unsafe { char::from_u32_unchecked(decode_utf8_2(self.to_utf8()) as u32) }
    }

    pub const fn is_vowel(self) -> bool {
        use utf8::*;
        matches!(self.to_utf8(), А | Е | И | О | У | Ы | Э | Ю | Я | Ё)
    }
    pub const fn is_hissing(self) -> bool {
        use utf8::*;
        matches!(self.to_utf8(), Ж | Ч | Ш | Щ)
    }
    pub const fn is_sibilant(self) -> bool {
        use utf8::*;
        matches!(self.to_utf8(), Ж | Ц | Ч | Ш | Щ)
    }
    pub const fn is_non_sibilant_consonant(self) -> bool {
        use utf8::*;
        matches!(self.to_utf8(), Б | В | Г | Д | З | Й | К | Л | М | Н | П | Р | С | Т | Ф | Х)
    }
    #[rustfmt::skip]
    pub const fn is_consonant(self) -> bool {
        use utf8::*;
        matches!(self.to_utf8(), Б | В | Г | Д | Ж | З | Й | К | Л | М | Н | П | Р | С | Т | Ф | Х | Ц | Ч | Ш | Щ)
    }

    pub const unsafe fn from_str_unchecked(s: &str) -> &[Utf8Letter] {
        unsafe { std::slice::from_raw_parts(s.as_ptr().cast(), s.len() / 2) }
    }
    pub fn from_str(s: &str) -> Option<&[Utf8Letter]> {
        if s.len() & 1 == 1 {
            return None;
        }
        let chunks = unsafe { s.as_bytes().as_chunks_unchecked::<2>() };
        if chunks.iter().all(|x| utf8::is_defined(*x)) {
            Some(unsafe { Self::from_str_unchecked(s) })
        } else {
            None
        }
    }

    pub const fn split_last(s: &str) -> Option<(&str, Utf8Letter)> {
        if let Some((remaining, last)) = s.as_bytes().split_last_chunk::<2>()
            && let Some(last) = Self::from_utf8(*last)
        {
            return Some((unsafe { str::from_utf8_unchecked(remaining) }, last));
        }
        None
    }
}

pub const trait Utf8LetterExt {
    fn as_str(&self) -> &str;
    fn as_mut_str(&mut self) -> &mut str;
}

impl const Utf8LetterExt for [Utf8Letter] {
    fn as_str(&self) -> &str {
        unsafe { std::str::from_raw_parts(self.as_ptr().cast(), self.len() * 2) }
    }
    fn as_mut_str(&mut self) -> &mut str {
        unsafe { std::str::from_raw_parts_mut(self.as_mut_ptr().cast(), self.len() * 2) }
    }
}

#[allow(dead_code)]
pub(crate) mod utf8 {
    use super::Utf8Letter;

    macro_rules! define_consts {
        ($($letter:ident)+) => ($(
            pub(crate) const $letter: [u8; 2] = Utf8Letter::$letter.to_utf8();
        )+);
    }
    define_consts! { А Б В Г Д Е Ж З И Й К Л М Н О П Р С Т У Ф Х Ц Ч Ш Щ Ъ Ы Ь Э Ю Я Ё }

    pub(crate) const fn is_defined(utf8: [u8; 2]) -> bool {
        //                 [А..=П]      |      [Р..=Я | Ё]
        matches!(utf8, [208, 176..=191] | [209, 128..=143 | 145])
    }
}
