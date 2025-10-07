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

#[allow(unnecessary_transmutes, reason = "u16::from_ne_bytes breaks constants in pop-up window")]
const fn d(ch: char) -> u16 {
    assert!(matches!(ch, 'а'..='я' | 'ё'));
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
    #[must_use]
    pub const fn from_utf8(utf8: [u8; 2]) -> Option<Self> {
        if is_lowercase_cyrillic(utf8) {
            Some(unsafe { Self::from_utf8_unchecked(utf8) })
        } else {
            None
        }
    }
    #[must_use]
    pub const unsafe fn from_utf8_unchecked(utf8: [u8; 2]) -> Self {
        debug_assert!(is_lowercase_cyrillic(utf8));
        unsafe { std::mem::transmute(utf8) }
    }

    #[must_use]
    pub const fn from_char(ch: char) -> Option<Self> {
        if matches!(ch, 'а'..='я' | 'ё') {
            Some(unsafe { Self::from_char_unchecked(ch) })
        } else {
            None
        }
    }
    #[must_use]
    pub const unsafe fn from_char_unchecked(ch: char) -> Self {
        debug_assert!(matches!(ch, 'а'..='я' | 'ё'));
        unsafe { Self::from_utf8_unchecked(encode_utf8_2(ch as u16)) }
    }

    #[must_use]
    pub const fn to_utf8(self) -> [u8; 2] {
        unsafe { std::mem::transmute(self) }
    }
    #[must_use]
    pub const fn as_utf8(&self) -> &[u8; 2] {
        unsafe { std::mem::transmute(self) }
    }
    #[must_use]
    pub const fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_utf8()) }
    }

    #[must_use]
    pub const fn to_char(self) -> char {
        unsafe { char::from_u32_unchecked(decode_utf8_2(self.to_utf8()) as u32) }
    }

    #[must_use]
    const fn last_byte(self) -> LetterLastByte {
        unsafe { std::mem::transmute(self.to_utf8()[1]) }
    }

    #[must_use]
    pub const fn is_vowel(self) -> bool {
        use LetterLastByte::*;
        matches!(self.last_byte(), А | Е | И | О | У | Ы | Э | Ю | Я | Ё)
    }
    #[must_use]
    pub const fn is_hissing(self) -> bool {
        use LetterLastByte::*;
        matches!(self.last_byte(), Ж | Ч | Ш | Щ)
    }
    #[must_use]
    pub const fn is_sibilant(self) -> bool {
        use LetterLastByte::*;
        matches!(self.last_byte(), Ж | Ц | Ч | Ш | Щ)
    }
    #[must_use]
    pub const fn is_non_sibilant_consonant(self) -> bool {
        use LetterLastByte::*;
        matches!(self.last_byte(), Б | В | Г | Д | З | Й | К | Л | М | Н | П | Р | С | Т | Ф | Х)
    }
    #[must_use]
    #[rustfmt::skip]
    pub const fn is_consonant(self) -> bool {
        use LetterLastByte::*;
        matches!(self.last_byte(), Б | В | Г | Д | Ж | З | Й | К | Л | М | Н | П | Р | С | Т | Ф | Х | Ц | Ч | Ш | Щ)
    }

    #[must_use]
    pub(crate) const fn is_stem_trim_letter(self) -> bool {
        use LetterLastByte::*;
        matches!(self.last_byte(), А | Е | И | Й | О | У | Ы | Ь | Э | Ю | Я | Ё)
    }

    #[must_use]
    pub const fn split_last(s: &str) -> Option<(&str, Utf8Letter)> {
        if let Some((remaining, last)) = s.as_bytes().split_last_chunk::<2>()
            && let Some(last) = Self::from_utf8(*last)
        {
            return Some((unsafe { str::from_utf8_unchecked(remaining) }, last));
        }
        None
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

pub const trait Utf8LetterSlice {
    #[must_use]
    fn as_str(&self) -> &str;
}

impl const Utf8LetterSlice for [Utf8Letter] {
    fn as_str(&self) -> &str {
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
