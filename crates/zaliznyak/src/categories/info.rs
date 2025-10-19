use crate::categories::{
    Animacy, Case, Gender, Number, Person, Tense,
    traits::{IntoAnimacy, IntoCase, IntoGender, IntoNumber, IntoPerson, IntoTense},
};

/// Standard declension parameters: [`Case`], [`Number`], [`Gender`], [`Animacy`].
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub struct DeclInfo {
    pub case: Case,
    pub number: Number,
    pub gender: Gender,
    pub animacy: Animacy,
}

impl const IntoCase for DeclInfo {
    fn case(&self) -> Case {
        self.case
    }
}
impl const IntoNumber for DeclInfo {
    fn number(&self) -> Number {
        self.number
    }
}
impl const IntoGender for DeclInfo {
    fn gender(&self) -> Gender {
        self.gender
    }
}
impl const IntoAnimacy for DeclInfo {
    fn animacy(&self) -> Animacy {
        self.animacy
    }
}

/// Standard conjugation parameters: [`Tense`], [`Number`], [`Gender`], [`Person`].
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub struct ConjInfo {
    pub tense: Tense,
    pub number: Number,
    pub gender: Gender,
    pub person: Person,
}

impl const IntoTense for ConjInfo {
    fn tense(&self) -> Tense {
        self.tense
    }
}
impl const IntoNumber for ConjInfo {
    fn number(&self) -> Number {
        self.number
    }
}
impl const IntoGender for ConjInfo {
    fn gender(&self) -> Gender {
        self.gender
    }
}
impl const IntoPerson for ConjInfo {
    fn person(&self) -> Person {
        self.person
    }
}
