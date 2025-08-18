use crate::{
    categories::{Case, Gender, Info, IntoNumber, Number},
    stress::{
        AdjectiveFullStress, AdjectiveShortStress, AdjectiveStress, AnyDualStress, AnyStress,
        NounStress, PronounStress, VerbPastStress, VerbPresentStress, VerbStress,
    },
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

impl NounStress {
    pub const fn is_stem_stressed(self, info: Info) -> bool {
        // Note: `is_nom_or_acc_inan` is called only when number is plural, i.e. when the
        // accusative case always maps to either nominative or genitive depending on animacy.

        match self {
            Self::A => true,
            Self::B => false,
            Self::C => info.is_singular(),
            Self::D => info.is_plural(),
            Self::E => info.is_singular() || info.case.is_nom_or_acc_inan(info),
            Self::F => info.is_plural() && info.case.is_nom_or_acc_inan(info),
            Self::Bp => info.is_singular() && info.case == Case::Instrumental,
            Self::Dp => info.is_plural() || info.case == Case::Accusative,
            Self::Fp => match info.number {
                Number::Singular => info.case == Case::Accusative,
                Number::Plural => info.case.is_nom_or_acc_inan(info),
            },
            Self::Fpp => match info.number {
                Number::Singular => info.case == Case::Instrumental,
                Number::Plural => info.case.is_nom_or_acc_inan(info),
            },
        }
    }
    pub const fn is_ending_stressed(self, info: Info) -> bool {
        !self.is_stem_stressed(info)
    }
}

impl PronounStress {
    pub const fn is_stem_stressed(self, info: Info) -> bool {
        match self {
            Self::A => true,
            Self::B => false,
            Self::F => info.is_plural() && info.case.is_nom_or_acc_inan(info),
        }
    }
    pub const fn is_ending_stressed(self, info: Info) -> bool {
        !self.is_stem_stressed(info)
    }
}

impl AdjectiveFullStress {
    pub const fn is_stem_stressed(self) -> bool {
        self == Self::A
    }
    pub const fn is_ending_stressed(self) -> bool {
        !self.is_stem_stressed()
    }
}
impl AdjectiveShortStress {
    pub const fn is_stem_stressed(self, gender: Gender, number: Number) -> Option<bool> {
        match self {
            Self::A => Some(true),
            Self::B => Some(number.is_singular() && gender == Gender::Masculine),
            Self::C => Some(number.is_plural() || gender != Gender::Feminine),

            Self::Ap => match (number, gender) {
                (Number::Plural, _) => Some(true),
                (_, Gender::Masculine) => Some(true),
                (_, Gender::Neuter) => Some(true),
                (_, Gender::Feminine) => None,
            },
            Self::Bp => match (number, gender) {
                (Number::Plural, _) => None,
                (_, Gender::Masculine) => Some(true),
                (_, Gender::Neuter) => Some(false),
                (_, Gender::Feminine) => Some(false),
            },
            Self::Cp => match (number, gender) {
                (Number::Plural, _) => None,
                (_, Gender::Masculine) => Some(true),
                (_, Gender::Neuter) => Some(true),
                (_, Gender::Feminine) => Some(false),
            },
            Self::Cpp => match (number, gender) {
                (Number::Plural, _) => None,
                (_, Gender::Masculine) => Some(true),
                (_, Gender::Neuter) => None,
                (_, Gender::Feminine) => Some(false),
            },
        }
    }
    pub const fn is_ending_stressed(self, gender: Gender, number: Number) -> Option<bool> {
        self.is_stem_stressed(gender, number).map(<bool as std::ops::Not>::not)
    }
}
