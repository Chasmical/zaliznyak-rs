use crate::stress::AnyStress;

impl AnyStress {
    pub const fn has_any_primes(self) -> bool {
        !matches!(self, Self::A | Self::B | Self::C | Self::D | Self::E | Self::F)
    }
    pub const fn has_single_prime(self) -> bool {
        matches!(self, Self::Ap | Self::Bp | Self::Cp | Self::Dp | Self::Ep | Self::Fp)
    }
    pub const fn has_double_prime(self) -> bool {
        matches!(self, Self::Cpp | Self::Fpp)
    }

    pub const fn unprime(self) -> AnyStress {
        match self {
            Self::A | Self::Ap => Self::A,
            Self::B | Self::Bp => Self::B,
            Self::C | Self::Cp | Self::Cpp => Self::C,
            Self::D | Self::Dp => Self::D,
            Self::E | Self::Ep => Self::E,
            Self::F | Self::Fp | Self::Fpp => Self::F,
        }
    }

    pub const fn add_single_prime(self) -> Option<AnyStress> {
        Some(match self {
            Self::A => Self::Ap,
            Self::B => Self::Bp,
            Self::C => Self::Cp,
            Self::D => Self::Dp,
            Self::E => Self::Ep,
            Self::F => Self::Fp,
            _ => return None,
        })
    }
    pub const fn add_double_prime(self) -> Option<AnyStress> {
        Some(match self {
            Self::C => Self::Cpp,
            Self::F => Self::Fpp,
            _ => return None,
        })
    }
}

// TODO: abbreviation methods (adjective/verb to any)

// TODO: normalization methods _adj/_verb (any dual to (any, any))

// TODO: is_stem_stressed methods
