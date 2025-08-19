use crate::stress::{AdjectiveStress, AnyDualStress, NounStress, PronounStress};

mod flags;
mod fmt;
mod from_str;
mod stem_types;

pub use flags::*;
pub use fmt::*;
pub use from_str::*;
pub use stem_types::*;

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum Declension {
    Noun(NounDeclension),
    Pronoun(PronounDeclension),
    Adjective(AdjectiveDeclension),
}

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum DeclensionKind {
    Noun,
    Pronoun,
    Adjective,
}

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub struct NounDeclension {
    pub stem_type: NounStemType,
    pub stress: NounStress,
    pub flags: DeclensionFlags,
}
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub struct PronounDeclension {
    pub stem_type: PronounStemType,
    pub stress: PronounStress,
    pub flags: DeclensionFlags,
}
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub struct AdjectiveDeclension {
    pub stem_type: AdjectiveStemType,
    pub stress: AdjectiveStress,
    pub flags: DeclensionFlags,
}

impl Declension {
    pub const fn is_noun(self) -> bool {
        matches!(self, Self::Noun(_))
    }
    pub const fn is_pronoun(self) -> bool {
        matches!(self, Self::Pronoun(_))
    }
    pub const fn is_adjective(self) -> bool {
        matches!(self, Self::Adjective(_))
    }
    pub const fn as_noun(self) -> Option<NounDeclension> {
        if let Self::Noun(x) = self { Some(x) } else { None }
    }
    pub const fn as_pronoun(self) -> Option<PronounDeclension> {
        if let Self::Pronoun(x) = self { Some(x) } else { None }
    }
    pub const fn as_adjective(self) -> Option<AdjectiveDeclension> {
        if let Self::Adjective(x) = self { Some(x) } else { None }
    }

    pub const fn kind(self) -> DeclensionKind {
        match self {
            Self::Noun(_) => DeclensionKind::Noun,
            Self::Pronoun(_) => DeclensionKind::Pronoun,
            Self::Adjective(_) => DeclensionKind::Adjective,
        }
    }
    pub const fn stem_type(self) -> AnyStemType {
        match self {
            Self::Noun(x) => x.stem_type.into(),
            Self::Pronoun(x) => x.stem_type.into(),
            Self::Adjective(x) => x.stem_type.into(),
        }
    }
    pub const fn stress(self) -> AnyDualStress {
        match self {
            Self::Noun(x) => x.stress.into(),
            Self::Pronoun(x) => x.stress.into(),
            Self::Adjective(x) => x.stress.into(),
        }
    }
    pub const fn flags(self) -> DeclensionFlags {
        match self {
            Self::Noun(x) => x.flags,
            Self::Pronoun(x) => x.flags,
            Self::Adjective(x) => x.flags,
        }
    }
}

impl const From<NounDeclension> for Declension {
    fn from(value: NounDeclension) -> Self {
        Self::Noun(value)
    }
}
impl const From<PronounDeclension> for Declension {
    fn from(value: PronounDeclension) -> Self {
        Self::Pronoun(value)
    }
}
impl const From<AdjectiveDeclension> for Declension {
    fn from(value: AdjectiveDeclension) -> Self {
        Self::Adjective(value)
    }
}

impl const TryFrom<Declension> for NounDeclension {
    type Error = ();
    fn try_from(value: Declension) -> Result<Self, Self::Error> {
        value.as_noun().ok_or(())
    }
}
impl const TryFrom<Declension> for PronounDeclension {
    type Error = ();
    fn try_from(value: Declension) -> Result<Self, Self::Error> {
        value.as_pronoun().ok_or(())
    }
}
impl const TryFrom<Declension> for AdjectiveDeclension {
    type Error = ();
    fn try_from(value: Declension) -> Result<Self, Self::Error> {
        value.as_adjective().ok_or(())
    }
}
