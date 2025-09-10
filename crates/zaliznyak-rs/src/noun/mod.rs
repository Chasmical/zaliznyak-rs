use crate::{
    categories::{Animacy, Gender, GenderEx, Number},
    declension::{AdjectiveStemType, Declension, NounStemType},
};
use thiserror::Error;

mod declension;
mod fmt;
mod from_str;

pub use from_str::*;

// FIXME(const-hack): Derive Clone and PartialEq with #[derive_const] when String supports it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Noun {
    stem: String,
    info: NounInfo,
}

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub struct NounInfo {
    pub declension: Option<Declension>,
    pub declension_gender: Gender,
    pub gender: GenderEx,
    pub animacy: Animacy,
    pub tantum: Option<Number>,
}

#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum NewNounError {
    #[error("invalid stem")]
    InvalidStem,
    #[error("not matching stem")]
    NotMatchingStemType,
}

impl Noun {
    pub const fn from_stem(stem: String, info: NounInfo) -> Self {
        Self { stem, info }
    }

    pub fn from_word(word: &str, info: NounInfo) -> Result<Self, NewNounError> {
        let stem = match info.declension {
            Some(Declension::Noun(decl)) => {
                let (stem, ty) = NounStemType::identify(word).ok_or(NewNounError::InvalidStem)?;

                if ty != decl.stem_type {
                    return Err(NewNounError::NotMatchingStemType);
                }
                stem
            },
            Some(Declension::Adjective(decl)) => {
                let (stem, ty, _) =
                    AdjectiveStemType::identify(word).ok_or(NewNounError::InvalidStem)?;

                if ty != decl.stem_type {
                    return Err(NewNounError::NotMatchingStemType);
                }
                stem
            },
            _ => word,
        };

        Ok(Self { stem: stem.to_owned(), info })
    }
}
