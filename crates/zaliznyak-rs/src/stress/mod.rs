pub mod convert;
mod methods;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnyStress {
    A = 1,
    B,
    C,
    D,
    E,
    F,
    Ap,
    Bp,
    Cp,
    Dp,
    Ep,
    Fp,
    Cpp,
    Fpp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NounStress {
    A,
    B,
    C,
    D,
    E,
    F,
    Bp,
    Dp,
    Fp,
    Fpp,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PronounStress {
    A,
    B,
    F,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdjectiveFullStress {
    A,
    B,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdjectiveShortStress {
    A,
    B,
    C,
    Ap,
    Bp,
    Cp,
    Cpp,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerbPresentStress {
    A,
    B,
    C,
    Cp,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerbPastStress {
    A,
    B,
    C,
    Cp,
    Cpp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnyDualStress {
    pub main: AnyStress,
    pub alt: Option<AnyStress>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdjectiveStress {
    pub full: AdjectiveFullStress,
    pub short: AdjectiveShortStress,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerbStress {
    pub present: VerbPresentStress,
    pub past: VerbPastStress,
}

impl AnyDualStress {
    pub const fn new(main: AnyStress, alt: Option<AnyStress>) -> Self {
        Self { main, alt }
    }
}
impl AdjectiveStress {
    pub const fn new(full: AdjectiveFullStress, short: AdjectiveShortStress) -> Self {
        Self { full, short }
    }
}
impl VerbStress {
    pub const fn new(present: VerbPresentStress, past: VerbPastStress) -> Self {
        Self { present, past }
    }
}
