use crate::util::enum_conversion;
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
