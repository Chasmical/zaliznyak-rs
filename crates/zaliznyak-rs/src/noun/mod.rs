use crate::{
    categories::{Animacy, Gender, GenderEx, Number},
    declension::Declension,
    word::WordBuf,
};
use thiserror::Error;

mod declension;
mod fmt;
mod from_str;

pub use from_str::*;

#[derive(Debug, Clone, Eq, Hash)]
#[derive_const(PartialEq)]
pub struct Noun {
    stem: WordBuf,
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
    pub const fn from_stem(stem: WordBuf, info: NounInfo) -> Self {
        Self { stem, info }
    }
}
