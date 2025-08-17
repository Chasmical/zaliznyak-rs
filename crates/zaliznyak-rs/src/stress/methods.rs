use crate::stress::{
    AdjectiveShortStress, AdjectiveStress, AnyDualStress, AnyStress, VerbPastStress,
    VerbPresentStress, VerbStress,
};

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

impl AnyDualStress {
    pub const fn normalize_adj(self) -> (AnyStress, AnyStress) {
        if let Some(alt) = self.alt { (self.main, alt) } else { (self.main.unprime(), self.main) }
    }
    pub const fn normalize_verb(self) -> (AnyStress, AnyStress) {
        (self.main, self.alt.unwrap_or(AnyStress::A))
    }

    pub const fn try_abbr_adj(self) -> Option<AnyStress> {
        if let Some(alt) = self.alt
            && !self.main.has_any_primes()
            && self.main as u8 == alt.unprime() as u8
        {
            return Some(alt);
        }
        None
    }
    pub const fn try_abbr_verb(self) -> Option<AnyStress> {
        // FIXME(const-hack): Replace with ==.
        if matches!(self.alt, Some(AnyStress::A)) { Some(self.main) } else { None }
    }

    pub const fn abbr_adj(self) -> AnyDualStress {
        self.try_abbr_adj().map_or(self, AnyDualStress::from)
    }
    pub const fn abbr_verb(self) -> AnyDualStress {
        self.try_abbr_verb().map_or(self, AnyDualStress::from)
    }
}

impl AdjectiveStress {
    pub const fn try_abbr(self) -> Option<AdjectiveShortStress> {
        match self {
            Self::A_A => Some(AdjectiveShortStress::A),
            Self::B_B => Some(AdjectiveShortStress::B),
            Self::A_Ap => Some(AdjectiveShortStress::Ap),
            Self::B_Bp => Some(AdjectiveShortStress::Bp),
            _ => None,
        }
    }
    pub const fn abbr(self) -> AnyDualStress {
        if let Some(abbr) = self.try_abbr() { abbr.into() } else { self.into() }
    }
}
impl VerbStress {
    pub const fn try_abbr(self) -> Option<VerbPresentStress> {
        // FIXME(const-hack): Replace with ==.
        if matches!(self.past, VerbPastStress::A) { Some(self.present) } else { None }
    }
    pub const fn abbr(self) -> AnyDualStress {
        if let Some(abbr) = self.try_abbr() { abbr.into() } else { self.into() }
    }
}

// TODO: is_stem_stressed methods
