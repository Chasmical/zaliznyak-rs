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

//                         TABLE OF STRESS TYPE CONVERSIONS
// ┌———————┬——————┬——————┬——————┬——————┬——————┬——————┬——————╥——————┬——————┬——————┐
// │From\To│ Any  │ Noun │ Pro  │ AdjF │ AdjS │ VerbF│ VerbP║ ANY  │ ADJ  │ VERB │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ Any   │ ———— │  []  │  []  │  []  │  []  │  []  │  []  ║  ██  │  []  │  []  │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ Noun  │  ██  │ ———— │      │      │      │      │      ║  ██  │      │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ Pro   │  ██  │      │ ———— │      │      │      │      ║  ██  │      │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ AdjF  │  ██  │      │      │ ———— │      │      │      ║  ██  │      │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ AdjS  │  ██  │      │      │      │ ———— │      │      ║  ██  │      │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ VerbF │  ██  │      │      │      │      │ ———— │      ║  ██  │      │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ VerbP │  ██  │      │      │      │      │      │ ———— ║  ██  │      │      │
// ╞═══════╪══════╪══════╪══════╪══════╪══════╪══════╪══════╬══════╪══════╪══════╡
// │ ANY   │  []  │  []  │  []  │  []  │  []  │  []  │  []  ║ ———— │  []  │  []  │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ ADJ   │      │      │      │      │      │      │      ║  ██  │ ———— │      │
// ├———————┼——————┼——————┼——————┼——————┼——————┼——————┼——————╫——————┼——————┼——————┤
// │ VERB  │      │      │      │      │      │      │      ║  ██  │      │ ———— │
// └———————┴——————┴——————┴——————┴——————┴——————┴——————┴——————╨——————┴——————┴——————┘
//                                                     ██ — From   [] — TryFrom

// Convert simple stresses to AnyStress, and vice versa
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

// Convert simple stresses to AnyDualStress
impl<T: [const] Into<AnyStress>> const From<T> for AnyDualStress {
    fn from(value: T) -> Self {
        Self::new(value.into(), None)
    }
}

// Convert AnyDualStress to simple stresses
impl const TryFrom<AnyDualStress> for AnyStress {
    type Error = ();
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        if value.alt.is_none() { Ok(value.main) } else { Err(()) }
    }
}
macro_rules! derive_simple_try_from_dual_impls {
    ($($t:ty),+ $(,)?) => ($(
        impl const TryFrom<AnyDualStress> for $t {
            type Error = <$t as TryFrom<AnyStress>>::Error;
            fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
                if value.alt.is_none() { value.main.try_into() } else { Err(Self::Error {}) }
            }
        }
    )+);
}
derive_simple_try_from_dual_impls! {
    NounStress, PronounStress, AdjectiveFullStress, AdjectiveShortStress, VerbPresentStress, VerbPastStress,
}

// Convert adjective/verb stresses to AnyDualStress
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

// Convert AnyDualStress to adjective/verb stresses
impl const TryFrom<AnyDualStress> for AdjectiveStress {
    type Error = AdjectiveStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        let (main, alt) = value.normalize_adj();

        Ok(Self::new(
            main.try_into().map_err(Self::Error::Full)?,
            alt.try_into().map_err(Self::Error::Short)?,
        ))
    }
}
impl const TryFrom<AnyDualStress> for VerbStress {
    type Error = VerbStressError;
    fn try_from(value: AnyDualStress) -> Result<Self, Self::Error> {
        let (main, alt) = value.normalize_verb();

        Ok(Self::new(
            main.try_into().map_err(Self::Error::Present)?,
            alt.try_into().map_err(Self::Error::Past)?,
        ))
    }
}

// Convert tuples of AnyStress to AnyDualStress
impl const From<(AnyStress, Option<AnyStress>)> for AnyDualStress {
    fn from(value: (AnyStress, Option<AnyStress>)) -> Self {
        Self::new(value.0, value.1)
    }
}
impl const From<(AnyStress, AnyStress)> for AnyDualStress {
    fn from(value: (AnyStress, AnyStress)) -> Self {
        Self::new(value.0, Some(value.1))
    }
}
