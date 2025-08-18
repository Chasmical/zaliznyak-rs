mod convert;
mod fmt;
mod from_str;
mod methods;

pub use convert::*;
pub use from_str::*;

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
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

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
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
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum PronounStress {
    A,
    B,
    F,
}
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum AdjectiveFullStress {
    A,
    B,
}
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum AdjectiveShortStress {
    A,
    B,
    C,
    Ap,
    Bp,
    Cp,
    Cpp,
}
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum VerbPresentStress {
    A,
    B,
    C,
    Cp,
}
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum VerbPastStress {
    A,
    B,
    C,
    Cp,
    Cpp,
}

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub struct AnyDualStress {
    pub main: AnyStress,
    pub alt: Option<AnyStress>,
}
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub struct AdjectiveStress {
    pub full: AdjectiveFullStress,
    pub short: AdjectiveShortStress,
}
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
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

#[allow(non_upper_case_globals)]
impl AdjectiveStress {
    pub const A: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::A);
    pub const A_A: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::A);
    pub const A_B: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::B);
    pub const A_C: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::C);
    pub const A_Ap: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::Ap);
    pub const A_Bp: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::Bp);
    pub const A_Cp: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::Cp);
    pub const A_Cpp: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::Cpp);

    pub const B: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::B);
    pub const B_A: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::A);
    pub const B_B: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::B);
    pub const B_C: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::C);
    pub const B_Ap: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::Ap);
    pub const B_Bp: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::Bp);
    pub const B_Cp: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::Cp);
    pub const B_Cpp: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::Cpp);

    pub const Ap: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::Ap);
    pub const Bp: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::Bp);
}
#[allow(non_upper_case_globals)]
impl VerbStress {
    pub const A: Self = Self::new(VerbPresentStress::A, VerbPastStress::A);
    pub const A_A: Self = Self::new(VerbPresentStress::A, VerbPastStress::A);
    pub const A_B: Self = Self::new(VerbPresentStress::A, VerbPastStress::B);
    pub const A_C: Self = Self::new(VerbPresentStress::A, VerbPastStress::C);
    pub const A_Cp: Self = Self::new(VerbPresentStress::A, VerbPastStress::Cp);
    pub const A_Cpp: Self = Self::new(VerbPresentStress::A, VerbPastStress::Cpp);

    pub const B: Self = Self::new(VerbPresentStress::B, VerbPastStress::A);
    pub const B_A: Self = Self::new(VerbPresentStress::B, VerbPastStress::A);
    pub const B_B: Self = Self::new(VerbPresentStress::B, VerbPastStress::B);
    pub const B_C: Self = Self::new(VerbPresentStress::B, VerbPastStress::C);
    pub const B_Cp: Self = Self::new(VerbPresentStress::B, VerbPastStress::Cp);
    pub const B_Cpp: Self = Self::new(VerbPresentStress::B, VerbPastStress::Cpp);

    pub const C: Self = Self::new(VerbPresentStress::C, VerbPastStress::A);
    pub const C_A: Self = Self::new(VerbPresentStress::C, VerbPastStress::A);
    pub const C_B: Self = Self::new(VerbPresentStress::C, VerbPastStress::B);
    pub const C_C: Self = Self::new(VerbPresentStress::C, VerbPastStress::C);
    pub const C_Cp: Self = Self::new(VerbPresentStress::C, VerbPastStress::Cp);
    pub const C_Cpp: Self = Self::new(VerbPresentStress::C, VerbPastStress::Cpp);

    pub const Cp: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::A);
    pub const Cp_A: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::A);
    pub const Cp_B: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::B);
    pub const Cp_C: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::C);
    pub const Cp_Cp: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::Cp);
    pub const Cp_Cpp: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::Cpp);
}
