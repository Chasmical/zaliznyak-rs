use crate::{declension::Declension, word::WordBuf};
use thiserror::Error;

mod declension;
mod flags;
mod fmt;

pub use flags::*;

// FIXME(const-hack): Derive PartialEq with #[derive_const] when String supports it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Adjective {
    stem: WordBuf,
    info: AdjectiveInfo,
}

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
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
    pub const fn from_stem(stem: WordBuf, info: AdjectiveInfo) -> Self {
        Self { stem, info }
    }
}
