use crate::{declension::Declension, word::WordBuf};
use thiserror::Error;

mod declension;

#[derive(Debug, Clone, Eq, Hash)]
#[derive_const(PartialEq)]
pub struct Pronoun {
    stem: WordBuf,
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
    pub const fn from_stem(stem: WordBuf, info: PronounInfo) -> Self {
        Self { stem, info }
    }
}
