use crate::{
    categories::{Animacy, Case, CaseEx, Gender, GenderEx, IntoAnimacy, Number},
    util::enum_conversion,
};
use thiserror::Error;

#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
#[error(
    "case must be one of the main 6: nominative, genitive, dative, accusative, instrumental or prepositional"
)]
pub struct CaseError;

#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
#[error("gender must be one of the main 3: masculine, neuter or feminine")]
pub struct GenderError;

enum_conversion! {
    CaseEx => Case { Nominative, Genitive, Dative, Accusative, Instrumental, Prepositional } else { CaseError }
}
enum_conversion! {
    GenderEx => Gender { Masculine, Neuter, Feminine } else { GenderError }
}

impl CaseEx {
    #[must_use]
    pub const fn normalize_with(self, number: Number) -> (Case, Number) {
        match self {
            Self::Partitive => (Case::Genitive, number),
            Self::Translative => (Case::Nominative, Number::Plural),
            Self::Locative => (Case::Prepositional, number),
            _ => (self.try_into().ok().unwrap(), number),
        }
    }
}

impl GenderEx {
    #[must_use]
    pub const fn normalize(self) -> Gender {
        self.try_into().unwrap_or(Gender::Feminine)
    }
}

impl Case {
    #[must_use]
    pub const fn is_nom_or_acc_inan<A>(self, info: A) -> bool
    where A: [const] IntoAnimacy + [const] std::marker::Destruct {
        self == Self::Nominative || self == Self::Accusative && info.is_inanimate()
    }
    #[must_use]
    pub const fn is_gen_or_acc_an<A>(self, info: A) -> bool
    where A: [const] IntoAnimacy + [const] std::marker::Destruct {
        self == Self::Genitive || self == Self::Accusative && info.is_animate()
    }
}

impl Animacy {
    #[must_use]
    pub const fn acc_case(self) -> Case {
        // Note: Free conversion! 0 => 0, 1 => 1.
        match self {
            Self::Inanimate => Case::Nominative,
            Self::Animate => Case::Genitive,
        }
    }
}
