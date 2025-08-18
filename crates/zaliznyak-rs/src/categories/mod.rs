mod convert;
mod info;
mod methods;
mod traits;

pub use convert::*;
pub use info::*;
pub use traits::*;

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub enum CaseEx {
    #[default]
    Nominative = 0,
    Genitive = 1,
    Dative = 2,
    Accusative = 3,
    Instrumental = 4,
    Prepositional = 5,
    Partitive = 6,
    Translative = 7,
    Locative = 8,
}
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub enum Case {
    #[default]
    Nominative = 0,
    Genitive = 1,
    Dative = 2,
    Accusative = 3,
    Instrumental = 4,
    Prepositional = 5,
}

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub enum GenderEx {
    #[default]
    Masculine = 0,
    Neuter = 1,
    Feminine = 2,
    Common = 3,
}
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub enum Gender {
    #[default]
    Masculine = 0,
    Neuter = 1,
    Feminine = 2,
}

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub enum Animacy {
    #[default]
    Inanimate = 0,
    Animate = 1,
}
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub enum Number {
    #[default]
    Singular = 0,
    Plural = 1,
}

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub enum Tense {
    #[default]
    Present,
    Past,
}
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub enum Person {
    #[default]
    First,
    Second,
    Third,
}
