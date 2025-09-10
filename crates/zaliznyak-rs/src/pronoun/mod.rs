use crate::declension::{AdjectiveStemType, Declension, PronounStemType};
use thiserror::Error;

mod declension;

// FIXME(const-hack): Derive Clone and PartialEq with #[derive_const] when String supports it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pronoun {
    stem: String,
    info: PronounInfo,
}

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub struct PronounInfo {
    pub declension: Option<Declension>,
}

#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum NewPronounError {
    #[error("invalid stem")]
    InvalidStem,
    #[error("not matching stem")]
    NotMatchingStemType,
}

impl Pronoun {
    pub const fn from_stem(stem: String, info: PronounInfo) -> Self {
        Self { stem, info }
    }

    pub fn from_word(word: &str, info: PronounInfo) -> Result<Self, NewPronounError> {
        let stem = match info.declension {
            Some(Declension::Pronoun(decl)) => {
                let (stem, ty) =
                    PronounStemType::identify(word).ok_or(NewPronounError::InvalidStem)?;

                if ty != decl.stem_type {
                    return Err(NewPronounError::NotMatchingStemType);
                }
                stem
            },
            Some(Declension::Adjective(decl)) => {
                let (stem, ty, _) =
                    AdjectiveStemType::identify(word).ok_or(NewPronounError::InvalidStem)?;

                if ty != decl.stem_type {
                    return Err(NewPronounError::NotMatchingStemType);
                }
                stem
            },
            _ => word,
        };

        Ok(Self { stem: stem.to_owned(), info })
    }
}
