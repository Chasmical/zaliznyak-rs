use crate::declension::{AdjectiveStemType, Declension, PronounStemType};
use thiserror::Error;

mod declension;
mod flags;
mod fmt;

pub use flags::*;

// FIXME(const-hack): Derive Clone and PartialEq with #[derive_const] when String supports it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Adjective {
    stem: String,
    info: AdjectiveInfo,
}

// FIXME(const-hack): Derive PartialEq with #[derive_const] when AdjectiveFlags supports it.
#[derive(Debug, Copy, PartialEq, Eq, Hash)]
#[derive_const(Clone)]
pub struct AdjectiveInfo {
    pub declension: Option<Declension>,
    pub flags: AdjectiveFlags,
    pub kind: AdjectiveKind,
}

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum AdjectiveKind {
    Regular,
    Pronoun,
    Numeral,
}

#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum NewAdjectiveError {
    #[error("invalid stem")]
    InvalidStem,
    #[error("not matching stem")]
    NotMatchingStemType,
}

impl Adjective {
    pub const fn from_stem(stem: String, info: AdjectiveInfo) -> Self {
        Self { stem, info }
    }

    pub fn from_word(word: &str, info: AdjectiveInfo) -> Result<Self, NewAdjectiveError> {
        let stem = match info.declension {
            Some(Declension::Adjective(decl)) => {
                let (stem, ty, _) =
                    AdjectiveStemType::identify(word).ok_or(NewAdjectiveError::InvalidStem)?;

                if ty != decl.stem_type {
                    return Err(NewAdjectiveError::NotMatchingStemType);
                }
                stem
            },
            Some(Declension::Pronoun(decl)) => {
                let (stem, ty) =
                    PronounStemType::identify(word).ok_or(NewAdjectiveError::InvalidStem)?;

                if ty != decl.stem_type {
                    return Err(NewAdjectiveError::NotMatchingStemType);
                }
                stem
            },
            _ => word,
        };

        Ok(Self { stem: stem.to_owned(), info })
    }
}
