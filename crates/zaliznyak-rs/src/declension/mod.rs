use crate::stress::{AdjectiveStress, AnyDualStress, NounStress, PronounStress};

mod endings;
mod endings_tables;
mod flags;
mod fmt;
mod from_str;
mod stem_types;

pub use flags::*;
pub use fmt::*;
pub use from_str::*;
pub use stem_types::*;

/// Any word declension type.
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum Declension {
    /// A noun declension. See [`NounDeclension`].
    Noun(NounDeclension),
    /// A pronoun declension. See [`PronounDeclension`].
    Pronoun(PronounDeclension),
    /// An adjective declension. See [`AdjectiveDeclension`].
    Adjective(AdjectiveDeclension),
}

/// A type of word declension.
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum DeclensionKind {
    /// A noun declension. See [`NounDeclension`].
    Noun,
    /// A pronoun declension. See [`PronounDeclension`].
    Pronoun,
    /// An adjective declension. See [`AdjectiveDeclension`].
    Adjective,
}

/// A noun declension.
///
/// # Examples
///
/// ```
/// use zaliznyak::{
///     declension::{DeclensionFlags, NounDeclension, NounStemType},
///     stress::NounStress,
/// };
///
/// let decl: NounDeclension = "2*d, ё".parse().unwrap();
///
/// assert_eq!(decl, NounDeclension {
///     stem_type: NounStemType::Type2,
///     stress: NounStress::D,
///     flags: DeclensionFlags::STAR | DeclensionFlags::ALTERNATING_YO,
/// });
/// ```
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub struct NounDeclension {
    /// The declension's stem type.
    pub stem_type: NounStemType,
    /// The declension's stress schema.
    pub stress: NounStress,
    /// The declension's flags.
    pub flags: DeclensionFlags,
}

/// A pronoun declension.
///
/// # Examples
///
/// ```
/// use zaliznyak::{
///     declension::{DeclensionFlags, PronounDeclension, PronounStemType},
///     stress::PronounStress,
/// };
///
/// let decl: PronounDeclension = "6*b".parse().unwrap();
///
/// assert_eq!(decl, PronounDeclension {
///     stem_type: PronounStemType::Type6,
///     stress: PronounStress::B,
///     flags: DeclensionFlags::STAR,
/// });
/// ```
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub struct PronounDeclension {
    /// The declension's stem type.
    pub stem_type: PronounStemType,
    /// The declension's stress schema.
    pub stress: PronounStress,
    /// The declension's flags.
    pub flags: DeclensionFlags,
}

/// An adjective declension.
///
/// # Examples
///
/// ```
/// use zaliznyak::{
///     declension::{AdjectiveDeclension, AdjectiveStemType, DeclensionFlags},
///     stress::AdjectiveStress,
/// };
///
/// let decl: AdjectiveDeclension = "3*a/c''".parse().unwrap();
///
/// assert_eq!(decl, AdjectiveDeclension {
///     stem_type: AdjectiveStemType::Type3,
///     stress: AdjectiveStress::A_Cpp,
///     flags: DeclensionFlags::STAR,
/// });
/// ```
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub struct AdjectiveDeclension {
    /// The declension's stem type.
    pub stem_type: AdjectiveStemType,
    /// The declension's stress schema.
    pub stress: AdjectiveStress,
    /// The declension's flags.
    pub flags: DeclensionFlags,
}

impl Declension {
    /// Returns `true` if this declension is a noun declension.
    pub const fn is_noun(self) -> bool {
        matches!(self, Self::Noun(_))
    }
    /// Returns `true` if this declension is a pronoun declension.
    pub const fn is_pronoun(self) -> bool {
        matches!(self, Self::Pronoun(_))
    }
    /// Returns `true` if this declension is an adjective declension.
    pub const fn is_adjective(self) -> bool {
        matches!(self, Self::Adjective(_))
    }
    /// Returns this declension as a noun declension, or `None` if it's of a different type.
    pub const fn as_noun(self) -> Option<NounDeclension> {
        if let Self::Noun(x) = self { Some(x) } else { None }
    }
    /// Returns this declension as a pronoun declension, or `None` if it's of a different type.
    pub const fn as_pronoun(self) -> Option<PronounDeclension> {
        if let Self::Pronoun(x) = self { Some(x) } else { None }
    }
    /// Returns this declension as an adjective declension, or `None` if it's of a different type.
    pub const fn as_adjective(self) -> Option<AdjectiveDeclension> {
        if let Self::Adjective(x) = self { Some(x) } else { None }
    }

    /// Returns this declension's type.
    pub const fn kind(self) -> DeclensionKind {
        match self {
            Self::Noun(_) => DeclensionKind::Noun,
            Self::Pronoun(_) => DeclensionKind::Pronoun,
            Self::Adjective(_) => DeclensionKind::Adjective,
        }
    }
    /// Returns this declension's stem type.
    pub const fn stem_type(self) -> AnyStemType {
        match self {
            Self::Noun(x) => x.stem_type.into(),
            Self::Pronoun(x) => x.stem_type.into(),
            Self::Adjective(x) => x.stem_type.into(),
        }
    }
    /// Returns this declension's stress schema.
    pub const fn stress(self) -> AnyDualStress {
        match self {
            Self::Noun(x) => x.stress.into(),
            Self::Pronoun(x) => x.stress.into(),
            Self::Adjective(x) => x.stress.into(),
        }
    }
    /// Returns this declension's flags.
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
