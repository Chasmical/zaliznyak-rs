use crate::{
    categories::{Animacy, Gender, GenderEx, Number},
    declension::Declension,
};

mod declension;
mod fmt;

// FIXME(const-hack): Derive Clone and PartialEq with #[derive_const] when String supports it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Noun {
    stem: String,
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
