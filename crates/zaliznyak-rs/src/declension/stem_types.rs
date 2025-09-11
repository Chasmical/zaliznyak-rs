use crate::{alphabet::Letter, util::enum_conversion};
use thiserror::Error;

macro_rules! impl_stem_type {
    (
        $(#[$outer:meta])*
        $vis:vis enum $T:ident {
            $( $(#[$inner:meta])* $variant:ident = $value:expr ),+ $(,)?
        }
    ) => (
        $(#[$outer])*
        #[derive(Debug, Copy, Eq, Hash)]
        #[derive_const(Clone, PartialEq)]
        $vis enum $T {
            $( $(#[$inner])* $variant,)+
        }

        impl $T {
            pub const fn from_digit(digit: u8) -> Option<Self> {
                Some(match digit { $($value => <$T>::$variant,)+ _ => return None })
            }
            pub const fn from_ascii_digit(ascii_digit: u8) -> Option<Self> {
                Self::from_digit(ascii_digit - b'0')
            }
            pub const fn to_digit(self) -> u8 {
                match self { $(<$T>::$variant => $value,)+ }
            }
            pub const fn to_ascii_digit(self) -> u8 {
                b'0' + self.to_digit()
            }
        }
        impl std::fmt::Display for $T {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let ascii_digit = self.to_ascii_digit();
                let slice = std::slice::from_ref(&ascii_digit);
                unsafe { str::from_utf8_unchecked(slice) }.fmt(f)
            }
        }
        impl std::str::FromStr for $T {
            type Err = ParseStemTypeError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if let [ch] = s.as_bytes() {
                    Ok(match ch - b'0' {
                        $($value => <$T>::$variant,)+
                        0..=9 => return Err(Self::Err::IncompatibleDigit),
                        _ => return Err(Self::Err::Invalid),
                    })
                } else {
                    Err(Self::Err::Invalid)
                }
            }
        }
    );
}

#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum ParseStemTypeError {
    #[error("digit not compatible with specified type")]
    IncompatibleDigit,
    #[error("invalid format")]
    Invalid,
}

#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
#[error("words can only have stem types 1 through 8")]
pub struct AnyStemTypeError;
#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
#[error("nouns can only have stem types 1 through 8")]
pub struct NounStemTypeError;
#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
#[error("pronouns can only have stem types 1, 2, 4 and 6")]
pub struct PronounStemTypeError;
#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
#[error("adjectives can only have stem types 1 through 7")]
pub struct AdjectiveStemTypeError;

impl_stem_type! {
    pub enum AnyStemType {
        Type1 = 1, Type2 = 2, Type3 = 3, Type4 = 4,
        Type5 = 5, Type6 = 6, Type7 = 7, Type8 = 8,
    }
}
impl_stem_type! {
    pub enum NounStemType {
        Type1 = 1, Type2 = 2, Type3 = 3, Type4 = 4,
        Type5 = 5, Type6 = 6, Type7 = 7, Type8 = 8,
    }
}
impl_stem_type! {
    pub enum PronounStemType {
        Type1 = 1, Type2 = 2, Type4 = 4, Type6 = 6,
    }
}
impl_stem_type! {
    pub enum AdjectiveStemType {
        Type1 = 1, Type2 = 2, Type3 = 3, Type4 = 4,
        Type5 = 5, Type6 = 6, Type7 = 7,
    }
}

enum_conversion! {
    NounStemType => AnyStemType { Type1, Type2, Type3, Type4, Type5, Type6, Type7, Type8 }
}
enum_conversion! {
    PronounStemType => AnyStemType { Type1, Type2, Type4, Type6 } else { PronounStemTypeError }
}
enum_conversion! {
    AdjectiveStemType => AnyStemType { Type1, Type2, Type3, Type4, Type5, Type6, Type7 }
    else { AdjectiveStemTypeError }
}

fn is_trim_letter(letter: Letter) -> bool {
    use Letter::*;
    matches!(letter, А | Е | И | Й | О | У | Ы | Ь | Э | Ю | Я | Ё)
}
fn identify_stem_type(stem: Letter, after: Option<Letter>) -> Option<AdjectiveStemType> {
    use {AdjectiveStemType as StemType, Letter::*};

    Some(match stem {
        Г | К | Х => StemType::Type3,
        Ж | Ч | Ш | Щ => StemType::Type4,
        Ц => StemType::Type5,
        А | Е | Й | О | У | Ы | Ь | Э | Ю | Я | Ё => StemType::Type6,
        И => StemType::Type7,
        Б | В | Д | З | Л | М | Н | П | Р | С | Т | Ф => {
            let hard = matches!(after, None | Some(А | О | У | Ы | Э));
            if hard { StemType::Type1 } else { StemType::Type2 }
        },
        Ъ => return None,
    })
}

impl NounStemType {
    pub fn identify(word: &str) -> Option<(&str, NounStemType)> {
        // Read the word's last char (must be in [а-яё] range)
        let last = Letter::from_char(word.chars().last()?)?;

        let (stem, stem_char, after) = {
            // If the last char is trimmable, exclude it from stem
            if is_trim_letter(last) {
                let stem = unsafe { word.get_unchecked(0..(word.len() - 2)) };
                // Read the actual last stem char (must be in [а-яё] range)
                let stem_char = Letter::from_char(stem.chars().last()?)?;
                (stem, stem_char, Some(last))
            } else {
                (word, last, None)
            }
        };

        // Identify the stem type from letters
        let stem_type = identify_stem_type(stem_char, after)?;
        Some((stem, AnyStemType::from(stem_type).into()))
    }
}

impl PronounStemType {
    pub fn identify(word: &str) -> Option<(&str, PronounStemType)> {
        let (stem, stem_type) = NounStemType::identify(word)?;
        Some((stem, AnyStemType::from(stem_type).try_into().ok()?))
    }
}

impl AdjectiveStemType {
    pub fn identify(word: &str) -> Option<(&str, AdjectiveStemType, bool)> {
        let (word, is_reflexive) = {
            // Remove 'ся' suffix from reflexive adjectives
            word.strip_suffix("ся").map_or((word, false), |x| (x, true))
        };

        let mut iter = word.chars().rev();

        // Read the word's two ending chars (both must be in [а-яё] range)
        _/*let ending_last_char*/ = Letter::from_char(iter.next()?)?;
        let ending_first_char = Letter::from_char(iter.next()?)?;

        // Read the stem's last char (must be in [а-яё] range)
        let stem_char = Letter::from_char(iter.next()?)?;

        // Ending is always 4 bytes (2 chars) long, so slicing is safe
        let stem = unsafe { word.get_unchecked(..(word.len() - 4)) };

        // Identify the stem type from letters
        let stem_type = identify_stem_type(stem_char, Some(ending_first_char))?;
        Some((stem, stem_type, is_reflexive))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identify_noun() {
        use NounStemType as ST;

        assert_eq!(ST::identify("ведро"), Some(("ведр", ST::Type1)));
        assert_eq!(ST::identify("тополь"), Some(("топол", ST::Type2)));
        assert_eq!(ST::identify("сапог"), Some(("сапог", ST::Type3)));
        assert_eq!(ST::identify("дача"), Some(("дач", ST::Type4)));
        assert_eq!(ST::identify("яйцо"), Some(("яйц", ST::Type5)));
        assert_eq!(ST::identify("бельё"), Some(("бель", ST::Type6)));
        assert_eq!(ST::identify("литий"), Some(("лити", ST::Type7)));

        assert_eq!(ST::identify("воръ"), None);
        assert_eq!(ST::identify("noun"), None);
        assert_eq!(ST::identify("nounя"), None);
    }

    #[test]
    fn identify_pro() {
        use PronounStemType as ST;

        assert_eq!(ST::identify("отцов"), Some(("отцов", ST::Type1)));
        assert_eq!(ST::identify("господень"), Some(("господен", ST::Type2)));
        assert_eq!(ST::identify("наш"), Some(("наш", ST::Type4)));
        assert_eq!(ST::identify("твой"), Some(("тво", ST::Type6)));

        assert_eq!(ST::identify("сёмга"), None); // stem type 3 - incompatible
        assert_eq!(ST::identify("блюдце"), None); // stem type 5 - incompatible
        assert_eq!(ST::identify("усилие"), None); // stem type 7 - incompatible
    }

    #[test]
    fn identify_adj() {
        use AdjectiveStemType as ST;

        assert_eq!(ST::identify("живой"), Some(("жив", ST::Type1, false)));
        assert_eq!(ST::identify("осенний"), Some(("осенн", ST::Type2, false)));
        assert_eq!(ST::identify("плавкий"), Some(("плавк", ST::Type3, false)));
        assert_eq!(ST::identify("светящийся"), Some(("светящ", ST::Type4, true)));
        assert_eq!(ST::identify("куцый"), Some(("куц", ST::Type5, false)));
        assert_eq!(ST::identify("голошеий"), Some(("голоше", ST::Type6, false)));
        assert_eq!(ST::identify("сиой"), Some(("си", ST::Type7, false))); // not a real adjective

        assert_eq!(ST::identify("серъое"), None);
        assert_eq!(ST::identify("adjective"), None);
        assert_eq!(ST::identify("adjectiveое"), None);
    }
}
