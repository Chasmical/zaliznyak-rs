use crate::{
    util::enum_conversion,
    word::{Utf8Letter, WordBuf},
};
use thiserror::Error;

macro_rules! impl_stem_type {
    (
        $(#[$outer:meta])*
        $vis:vis enum $T:ident {
            $( $(#[$inner:meta])* $variant:ident => $value:expr ),+ $(,)?
        }
    ) => (
        $(#[$outer])*
        #[derive(Debug, Copy, Eq, Hash)]
        #[derive_const(Clone, PartialEq)]
        #[allow(missing_docs)]
        $vis enum $T {
            $( $(#[$inner])* $variant,)+
        }

        impl $T {
            #[doc = concat!("Converts a digit to a [`", stringify!($T), "`].")]
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use zaliznyak::declension::", stringify!($T), ";")]
            ///
            #[doc = concat!("assert_eq!(", stringify!($T), "::from_digit(0), None);")]
            #[doc = concat!("assert_eq!(", stringify!($T), "::from_digit(4), Some(", stringify!($T), "::Type4));")]
            #[doc = concat!("assert_eq!(", stringify!($T), "::from_digit(9), None);")]
            /// ```
            pub const fn from_digit(digit: u8) -> Option<Self> {
                Some(match digit { $($value => <$T>::$variant,)+ _ => return None })
            }
            #[doc = concat!("Converts an ASCII digit to a [`", stringify!($T), "`].")]
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use zaliznyak::declension::", stringify!($T), ";")]
            ///
            #[doc = concat!("assert_eq!(", stringify!($T), "::from_ascii_digit(b'0'), None);")]
            #[doc = concat!("assert_eq!(", stringify!($T), "::from_ascii_digit(b'4'), Some(", stringify!($T), "::Type4));")]
            #[doc = concat!("assert_eq!(", stringify!($T), "::from_ascii_digit(b'9'), None);")]
            /// ```
            pub const fn from_ascii_digit(ascii_digit: u8) -> Option<Self> {
                Self::from_digit(ascii_digit - b'0')
            }
            /// Converts this stem type to its corresponding digit.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use zaliznyak::declension::", stringify!($T), ";")]
            ///
            #[doc = concat!("assert_eq!(", stringify!($T), "::Type1.to_digit(), 1);")]
            #[doc = concat!("assert_eq!(", stringify!($T), "::Type4.to_digit(), 4);")]
            #[doc = concat!("assert_eq!(", stringify!($T), "::Type6.to_digit(), 6);")]
            /// ```
            pub const fn to_digit(self) -> u8 {
                match self { $(<$T>::$variant => $value,)+ }
            }
            /// Converts this stem type to its corresponding ASCII digit.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use zaliznyak::declension::", stringify!($T), ";")]
            ///
            #[doc = concat!("assert_eq!(", stringify!($T), "::Type1.to_ascii_digit(), b'1');")]
            #[doc = concat!("assert_eq!(", stringify!($T), "::Type4.to_ascii_digit(), b'4');")]
            #[doc = concat!("assert_eq!(", stringify!($T), "::Type6.to_ascii_digit(), b'6');")]
            /// ```
            pub const fn to_ascii_digit(self) -> u8 {
                b'0' + self.to_digit()
            }
        }
        impl std::fmt::Display for $T {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Write::write_char(f, self.to_ascii_digit() as char)
            }
        }
        impl const std::str::FromStr for $T {
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

/// Error type for parsing various stem types.
#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum ParseStemTypeError {
    /// The parsed value is not compatible with specified stem type.
    #[error("digit not compatible with specified type")]
    IncompatibleDigit,
    /// Invalid format.
    #[error("invalid format")]
    Invalid,
}

/// Error type for conversion to [`AnyStemType`].
#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
#[error("words can only have stem types 1 through 8")]
pub struct AnyStemTypeError;
/// Error type for conversion to [`NounStemType`].
#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
#[error("nouns can only have stem types 1 through 8")]
pub struct NounStemTypeError;
/// Error type for conversion to [`PronounStemType`].
#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
#[error("pronouns can only have stem types 1, 2, 4 and 6")]
pub struct PronounStemTypeError;
/// Error type for conversion to [`AdjectiveStemType`].
#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
#[error("adjectives can only have stem types 1 through 6")]
pub struct AdjectiveStemTypeError;

impl_stem_type! {
    /// Any word's stem type. Can be converted to and from any other stem type.
    /// [See the dictionary for more details](https://gramdict.ru/declension/symbols#meaning1).
    pub enum AnyStemType {
        Type1 => 1, Type2 => 2, Type3 => 3, Type4 => 4,
        Type5 => 5, Type6 => 6, Type7 => 7, Type8 => 8,
    }
}
impl_stem_type! {
    /// A noun stem type.
    /// [See the dictionary for more details](https://gramdict.ru/declension/symbols#meaning1).
    pub enum NounStemType {
        Type1 => 1, Type2 => 2, Type3 => 3, Type4 => 4,
        Type5 => 5, Type6 => 6, Type7 => 7, Type8 => 8,
    }
}
impl_stem_type! {
    /// A pronoun stem type.
    /// [See the dictionary for more details](https://gramdict.ru/declension/symbols#meaning1).
    pub enum PronounStemType {
        Type1 => 1, Type2 => 2, Type4 => 4, Type6 => 6,
    }
}
impl_stem_type! {
    /// An adjective stem type.
    /// [See the dictionary for more details](https://gramdict.ru/declension/symbols#meaning1).
    pub enum AdjectiveStemType {
        Type1 => 1, Type2 => 2, Type3 => 3, Type4 => 4,
        Type5 => 5, Type6 => 6,
    }
}

enum_conversion! {
    AnyStemType => NounStemType { Type1, Type2, Type3, Type4, Type5, Type6, Type7, Type8 }
}
enum_conversion! {
    AnyStemType => PronounStemType { Type1, Type2, Type4, Type6 } else { PronounStemTypeError }
}
enum_conversion! {
    AnyStemType => AdjectiveStemType { Type1, Type2, Type3, Type4, Type5, Type6 }
    else { AdjectiveStemTypeError }
}

const fn identify_any(stem: Utf8Letter, after: Option<Utf8Letter>) -> Option<AnyStemType> {
    use Utf8Letter::*;

    Some(match stem {
        Г | К | Х => AnyStemType::Type3,
        Ж | Ч | Ш | Щ => AnyStemType::Type4,
        Ц => AnyStemType::Type5,
        А | Е | Й | О | У | Ы | Ь | Э | Ю | Я | Ё => AnyStemType::Type6,
        И => AnyStemType::Type7,
        Б | В | Д | З | Л | М | Н | П | Р | С | Т | Ф => {
            let hard = matches!(after, None | Some(А | О | У | Ы | Э));
            if hard { AnyStemType::Type1 } else { AnyStemType::Type2 }
        },
        Ъ => return None,
    })
}

impl NounStemType {
    /// Identifies a noun's stem and stem type from its nominative form.
    #[must_use]
    pub const fn identify(word: &[Utf8Letter]) -> Option<(&[Utf8Letter], NounStemType)> {
        // Read the word's last char
        let (&last, word_without_last) = word.split_last()?;

        let (stem, stem_char, after) = {
            // If the last char is trimmable, exclude it from stem
            if last.is_stem_trim_letter() {
                // Read the actual last stem char
                let &stem_char = word_without_last.last()?;
                (word_without_last, stem_char, Some(last))
            } else {
                (word, last, None)
            }
        };

        // Identify the stem type from letters
        let stem_type = identify_any(stem_char, after)?;
        Some((stem, stem_type.into()))
    }

    #[must_use]
    pub fn identify_trim(word: &mut WordBuf) -> Option<NounStemType> {
        match Self::identify(word.as_letters()) {
            Some((stem, ty)) => {
                let stem_len = stem.len();
                word.set_stem_len(stem_len);
                Some(ty)
            },
            None => None,
        }
    }
}

impl PronounStemType {
    /// Identifies a pronoun's stem and stem type from its nominative form.
    #[must_use]
    pub const fn identify(word: &[Utf8Letter]) -> Option<(&[Utf8Letter], PronounStemType)> {
        let (stem, stem_type) = NounStemType::identify(word)?;
        Some((stem, AnyStemType::from(stem_type).try_into().ok()?))
    }

    #[must_use]
    pub fn identify_trim(word: &mut WordBuf) -> Option<PronounStemType> {
        match Self::identify(word.as_letters()) {
            Some((stem, ty)) => {
                let stem_len = stem.len();
                word.set_stem_len(stem_len);
                Some(ty)
            },
            None => None,
        }
    }
}

impl AdjectiveStemType {
    /// Identifies an adjective's stem and stem type from its nominative form, also returning a
    /// `bool` indicating whether the adjective is reflexive or not.
    #[must_use]
    pub const fn identify(word: &[Utf8Letter]) -> Option<(&[Utf8Letter], AdjectiveStemType, bool)> {
        let (word, is_reflexive) = {
            // Remove 'ся' suffix from reflexive adjectives

            // FIXME(const-hack): Replace with `.strip_suffix().map_or((word, false), |x| (x, true))`.
            if let Some((stripped, last)) = word.split_last_chunk::<2>()
                && last == &[Utf8Letter::С, Utf8Letter::Я]
            {
                (stripped, true)
            } else {
                (word, false)
            }
        };

        // Read the word's two ending chars
        let (&_ending_last_char, word) = word.split_last()?;
        let (&ending_first_char, word) = word.split_last()?;

        // Read the stem's last char
        let &stem_char = word.last()?;

        // Identify the stem type from letters
        let stem_type = identify_any(stem_char, Some(ending_first_char))?;
        Some((word, stem_type.try_into().ok()?, is_reflexive))
    }

    #[must_use]
    pub fn identify_trim(word: &mut WordBuf) -> Option<(AdjectiveStemType, bool)> {
        match Self::identify(word.as_letters()) {
            Some((stem, ty, is_reflexive)) => {
                let stem_len = stem.len();
                word.set_stem_len(stem_len);
                Some((ty, is_reflexive))
            },
            None => None,
        }
    }
}
