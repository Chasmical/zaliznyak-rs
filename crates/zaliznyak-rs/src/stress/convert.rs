use crate::{
    stress::{
        AdjectiveFullStress, AdjectiveShortStress, AdjectiveStress, AnyDualStress, AnyStress,
        NounStress, PronounStress, VerbPastStress, VerbPresentStress, VerbStress,
    },
    util::enum_conversion,
};
use thiserror::Error;

#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("words can only have stresses a-f, a′-f′, c″ and f″")]
pub struct AnyStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("nouns can only have stresses a-f, b′, d′, f′ and f″")]
pub struct NounStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("pronouns can only have stresses a, b and f")]
pub struct PronounStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("adjectives (full form) can only have stresses a and b")]
pub struct AdjectiveFullStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("adjectives (short form) can only have stresses a-c, a′-c′ and c″")]
pub struct AdjectiveShortStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("verbs (present tense) can only have stresses a, b, c and c′")]
pub struct VerbPresentStressError;
#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("verbs (past tense) can only have stresses a, b, c, c′ and c″")]
pub struct VerbPastStressError;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum AdjectiveStressError {
    #[error("{0}")]
    Full(#[from] AdjectiveFullStressError),
    #[error("{0}")]
    Short(#[from] AdjectiveShortStressError),
}
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum VerbStressError {
    #[error("{0}")]
    Present(#[from] VerbPresentStressError),
    #[error("{0}")]
    Past(#[from] VerbPastStressError),
}

enum_conversion! {
    NounStress => AnyStress { A, B, C, D, E, F, Bp, Dp, Fp, Fpp } else { NounStressError }
}
enum_conversion! {
    PronounStress => AnyStress { A, B, F } else { PronounStressError }
}
enum_conversion! {
    AdjectiveFullStress => AnyStress { A, B } else { AdjectiveFullStressError }
}
enum_conversion! {
    AdjectiveShortStress => AnyStress { A, B, C, Ap, Bp, Cp, Cpp } else { AdjectiveShortStressError }
}
enum_conversion! {
    VerbPresentStress => AnyStress { A, B, C, Cp } else { VerbPresentStressError }
}
enum_conversion! {
    VerbPastStress => AnyStress { A, B, C, Cp, Cpp } else { VerbPastStressError }
}

impl<T: [const] Into<AnyStress>> const From<T> for AnyDualStress {
    fn from(value: T) -> Self {
        Self::new(value.into(), None)
    }
}
impl const From<AdjectiveStress> for AnyDualStress {
    fn from(value: AdjectiveStress) -> Self {
        Self::new(value.full.into(), Some(value.short.into()))
    }
}
impl const From<VerbStress> for AnyDualStress {
    fn from(value: VerbStress) -> Self {
        Self::new(value.present.into(), Some(value.past.into()))
    }
}
