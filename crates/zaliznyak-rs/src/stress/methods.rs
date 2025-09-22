use crate::{
    categories::{Case, DeclInfo, Gender, IntoNumber, IntoPerson, Number, Person},
    stress::{
        AdjectiveFullStress, AdjectiveShortStress, AdjectiveStress, AnyDualStress, AnyStress,
        NounStress, PronounStress, VerbPastStress, VerbPresentStress, VerbStress,
    },
};

impl AnyStress {
    /// Returns `true` if this stress is a primary letter stress, with no primes.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::AnyStress;
    ///
    /// assert_eq!(AnyStress::F.is_primary(), true);
    /// assert_eq!(AnyStress::Ap.is_primary(), false);
    /// assert_eq!(AnyStress::Cpp.is_primary(), false);
    /// ```
    #[must_use]
    pub const fn is_primary(self) -> bool {
        matches!(self, Self::A | Self::B | Self::C | Self::D | Self::E | Self::F)
    }
    /// Returns `true` if this stress has a single or double prime.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::AnyStress;
    ///
    /// assert_eq!(AnyStress::F.has_any_primes(), false);
    /// assert_eq!(AnyStress::Ap.has_any_primes(), true);
    /// assert_eq!(AnyStress::Cpp.has_any_primes(), true);
    /// ```
    #[must_use]
    pub const fn has_any_primes(self) -> bool {
        !self.is_primary()
    }
    /// Returns `true` if this stress has exactly one prime.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::AnyStress;
    ///
    /// assert_eq!(AnyStress::F.has_single_prime(), false);
    /// assert_eq!(AnyStress::Ap.has_single_prime(), true);
    /// assert_eq!(AnyStress::Cpp.has_single_prime(), false);
    /// ```
    #[must_use]
    pub const fn has_single_prime(self) -> bool {
        matches!(self, Self::Ap | Self::Bp | Self::Cp | Self::Dp | Self::Ep | Self::Fp)
    }
    /// Returns `true` if this stress has exactly two primes.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::AnyStress;
    ///
    /// assert_eq!(AnyStress::F.has_double_prime(), false);
    /// assert_eq!(AnyStress::Ap.has_double_prime(), false);
    /// assert_eq!(AnyStress::Cpp.has_double_prime(), true);
    /// ```
    #[must_use]
    pub const fn has_double_prime(self) -> bool {
        matches!(self, Self::Cpp | Self::Fpp)
    }

    /// Returns the primary letter stress, that this stress is based on.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::AnyStress;
    ///
    /// assert_eq!(AnyStress::F.unprime(), AnyStress::F);
    /// assert_eq!(AnyStress::Ap.unprime(), AnyStress::A);
    /// assert_eq!(AnyStress::Cpp.unprime(), AnyStress::C);
    /// ```
    #[must_use]
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

    /// Returns the single prime version of this primary letter stress. Returns `None` if this
    /// stress is not a primary letter stress.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::AnyStress;
    ///
    /// assert_eq!(AnyStress::F.add_single_prime(), Some(AnyStress::Fp));
    /// assert_eq!(AnyStress::Ap.add_single_prime(), None);
    /// assert_eq!(AnyStress::Cpp.add_single_prime(), None);
    /// ```
    #[must_use]
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
    /// Returns the double prime version of this primary letter stress. Returns `None` if this
    /// this stress is not a primary letter stress, or if there is no double prime version of it.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::AnyStress;
    ///
    /// assert_eq!(AnyStress::A.add_double_prime(), None);
    /// assert_eq!(AnyStress::F.add_double_prime(), Some(AnyStress::Fpp));
    /// assert_eq!(AnyStress::Ap.add_double_prime(), None);
    /// assert_eq!(AnyStress::Cpp.add_double_prime(), None);
    /// ```
    #[must_use]
    pub const fn add_double_prime(self) -> Option<AnyStress> {
        Some(match self {
            Self::C => Self::Cpp,
            Self::F => Self::Fpp,
            _ => return None,
        })
    }
}

impl AnyDualStress {
    /// Returns `true` if this dual stress has both stresses specified.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::{AnyDualStress, AnyStress};
    ///
    /// let x = AnyDualStress::new(AnyStress::A, None);
    /// assert_eq!(x.is_dual(), false);
    ///
    /// let x = AnyDualStress::new(AnyStress::B, Some(AnyStress::C));
    /// assert_eq!(x.is_dual(), true);
    /// ```
    #[must_use]
    pub const fn is_dual(self) -> bool {
        self.alt.is_some()
    }

    /// Normalizes this dual stress using adjective stress abbreviation rules.
    ///
    /// If both stresses are specified, then they are returned unchanged. Otherwise, the following
    /// conversion is applied: a --- a/a, b --- b/b, a′ --- a/a′, c″ --- c/c″, and etc.
    /// The result is not necessarily a valid [`AdjectiveStress`].
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::{AnyDualStress, AnyStress};
    ///
    /// // b′/c => b′/c (unchanged)
    /// let x = AnyDualStress::new(AnyStress::Bp, Some(AnyStress::C));
    /// assert_eq!(x.normalize_adj(), (AnyStress::Bp, AnyStress::C));
    ///
    /// // d => d/d
    /// let x = AnyDualStress::new(AnyStress::D, None);
    /// assert_eq!(x.normalize_adj(), (AnyStress::D, AnyStress::D));
    ///
    /// // b′ => b/b′
    /// let x = AnyDualStress::new(AnyStress::Bp, None);
    /// assert_eq!(x.normalize_adj(), (AnyStress::B, AnyStress::Bp));
    ///
    /// // c″ => c/c″
    /// let x = AnyDualStress::new(AnyStress::Cpp, None);
    /// assert_eq!(x.normalize_adj(), (AnyStress::C, AnyStress::Cpp));
    /// ```
    #[must_use]
    pub const fn normalize_adj(self) -> (AnyStress, AnyStress) {
        if let Some(alt) = self.alt { (self.main, alt) } else { (self.main.unprime(), self.main) }
    }
    /// Normalizes this dual stress using verb stress abbreviation rules.
    ///
    /// If alternative stress is unspecified, it is set to a.
    /// The result is not necessarily a valid [`VerbStress`].
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::{AnyDualStress, AnyStress};
    ///
    /// // b′/c => b′/c (unchanged)
    /// let x = AnyDualStress::new(AnyStress::Bp, Some(AnyStress::C));
    /// assert_eq!(x.normalize_verb(), (AnyStress::Bp, AnyStress::C));
    ///
    /// // c => c/a
    /// let x = AnyDualStress::new(AnyStress::C, None);
    /// assert_eq!(x.normalize_verb(), (AnyStress::C, AnyStress::A));
    ///
    /// // d′ => d′/a
    /// let x = AnyDualStress::new(AnyStress::Dp, None);
    /// assert_eq!(x.normalize_verb(), (AnyStress::Dp, AnyStress::A));
    ///
    /// // f″ => f″/a
    /// let x = AnyDualStress::new(AnyStress::Fpp, None);
    /// assert_eq!(x.normalize_verb(), (AnyStress::Fpp, AnyStress::A));
    /// ```
    #[must_use]
    pub const fn normalize_verb(self) -> (AnyStress, AnyStress) {
        (self.main, self.alt.unwrap_or(AnyStress::A))
    }

    /// Tries to abbreviate this dual stress using adjective stress abbreviation rules.
    ///
    /// If both stresses are specified, the following conversion is attempted: a/a --- a,
    /// b/b --- b, a/a′ --- a′, c/c″ --- c″, and etc. If this dual stress is already abbreviated,
    /// or if the conversion was unsuccessful, returns `None`.
    /// The result is not necessarily a valid [`AdjectiveStress`].
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::{AnyDualStress, AnyStress};
    ///
    /// // d/d => d
    /// let x = AnyDualStress::new(AnyStress::D, Some(AnyStress::D));
    /// assert_eq!(x.try_abbr_adj(), Some(AnyStress::D));
    ///
    /// // b/b′ => b′
    /// let x = AnyDualStress::new(AnyStress::B, Some(AnyStress::Bp));
    /// assert_eq!(x.try_abbr_adj(), Some(AnyStress::Bp));
    ///
    /// // c/c″ => c″
    /// let x = AnyDualStress::new(AnyStress::C, Some(AnyStress::Cpp));
    /// assert_eq!(x.try_abbr_adj(), Some(AnyStress::Cpp));
    ///
    /// // d′ is already abbreviated
    /// let x = AnyDualStress::new(AnyStress::Dp, None);
    /// assert_eq!(x.try_abbr_adj(), None);
    ///
    /// // b′/c cannot be abbreviated
    /// let x = AnyDualStress::new(AnyStress::Bp, Some(AnyStress::C));
    /// assert_eq!(x.try_abbr_adj(), None);
    /// ```
    #[must_use]
    pub const fn try_abbr_adj(self) -> Option<AnyStress> {
        if let Some(alt) = self.alt
            && self.main.is_primary()
            && self.main == alt.unprime()
        {
            return Some(alt);
        }
        None
    }
    /// Tries to abbreviate this dual stress using verb stress abbreviation rules.
    ///
    /// If both stresses are specified, the following conversion is attempted: a/a --- a,
    /// b/a --- b, a′/a --- a′, c″/a --- c″, and etc. If this dual stress is already abbreviated,
    /// or if the conversion was unsuccessful, returns `None`.
    /// The result is not necessarily a valid [`VerbStress`].
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::{AnyDualStress, AnyStress};
    ///
    /// // c/a => c
    /// let x = AnyDualStress::new(AnyStress::C, Some(AnyStress::A));
    /// assert_eq!(x.try_abbr_verb(), Some(AnyStress::C));
    ///
    /// // d′/a => d′
    /// let x = AnyDualStress::new(AnyStress::Dp, Some(AnyStress::A));
    /// assert_eq!(x.try_abbr_verb(), Some(AnyStress::Dp));
    ///
    /// // f″/a => f″
    /// let x = AnyDualStress::new(AnyStress::Fpp, Some(AnyStress::A));
    /// assert_eq!(x.try_abbr_verb(), Some(AnyStress::Fpp));
    ///
    /// // d′ is already abbreviated
    /// let x = AnyDualStress::new(AnyStress::Dp, None);
    /// assert_eq!(x.try_abbr_verb(), None);
    ///
    /// // b′/c cannot be abbreviated
    /// let x = AnyDualStress::new(AnyStress::Bp, Some(AnyStress::C));
    /// assert_eq!(x.try_abbr_verb(), None);
    /// ```
    #[must_use]
    pub const fn try_abbr_verb(self) -> Option<AnyStress> {
        if self.alt == Some(AnyStress::A) { Some(self.main) } else { None }
    }

    /// Tries to abbreviate this dual stress using adjective stress abbreviation rules.
    ///
    /// If both stresses are specified, the following conversion is attempted: a/a --- a,
    /// b/b --- b, a/a′ --- a′, c/c″ --- c″, and etc. If this dual stress is already abbreviated,
    /// or if the conversion was unsuccessful, returns this dual stress unchanged.
    /// The result is not necessarily a valid [`AdjectiveStress`].
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::{AnyDualStress, AnyStress};
    ///
    /// // d/d => d
    /// let x = AnyDualStress::new(AnyStress::D, Some(AnyStress::D));
    /// assert_eq!(x.abbr_adj(), AnyDualStress::new(AnyStress::D, None));
    ///
    /// // b/b′ => b′
    /// let x = AnyDualStress::new(AnyStress::B, Some(AnyStress::Bp));
    /// assert_eq!(x.abbr_adj(), AnyDualStress::new(AnyStress::Bp, None));
    ///
    /// // c/c″ => c″
    /// let x = AnyDualStress::new(AnyStress::C, Some(AnyStress::Cpp));
    /// assert_eq!(x.abbr_adj(), AnyDualStress::new(AnyStress::Cpp, None));
    ///
    /// // d′ is already abbreviated
    /// let x = AnyDualStress::new(AnyStress::Dp, None);
    /// assert_eq!(x.abbr_adj(), x);
    ///
    /// // b′/c cannot be abbreviated
    /// let x = AnyDualStress::new(AnyStress::Bp, Some(AnyStress::C));
    /// assert_eq!(x.abbr_adj(), x);
    /// ```
    #[must_use]
    pub const fn abbr_adj(self) -> AnyDualStress {
        self.try_abbr_adj().map_or(self, AnyDualStress::from)
    }
    /// Tries to abbreviate this dual stress using verb stress abbreviation rules.
    ///
    /// If both stresses are specified, the following conversion is attempted: a/a --- a,
    /// b/a --- b, a′/a --- a′, c″/a --- c″, and etc. If this dual stress is already abbreviated,
    /// or if the conversion was unsuccessful, returns this dual stress unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::{AnyDualStress, AnyStress};
    ///
    /// // c/a => c
    /// let x = AnyDualStress::new(AnyStress::C, Some(AnyStress::A));
    /// assert_eq!(x.abbr_verb(), AnyDualStress::new(AnyStress::C, None));
    ///
    /// // d′/a => d′
    /// let x = AnyDualStress::new(AnyStress::Dp, Some(AnyStress::A));
    /// assert_eq!(x.abbr_verb(), AnyDualStress::new(AnyStress::Dp, None));
    ///
    /// // f″/a => f″
    /// let x = AnyDualStress::new(AnyStress::Fpp, Some(AnyStress::A));
    /// assert_eq!(x.abbr_verb(), AnyDualStress::new(AnyStress::Fpp, None));
    ///
    /// // d′ is already abbreviated
    /// let x = AnyDualStress::new(AnyStress::Dp, None);
    /// assert_eq!(x.abbr_verb(), x);
    ///
    /// // b′/c cannot be abbreviated
    /// let x = AnyDualStress::new(AnyStress::Bp, Some(AnyStress::C));
    /// assert_eq!(x.abbr_verb(), x);
    /// ```
    #[must_use]
    pub const fn abbr_verb(self) -> AnyDualStress {
        self.try_abbr_verb().map_or(self, AnyDualStress::from)
    }
}

impl AdjectiveStress {
    /// Tries to abbreviate this dual adjective stress into [`AdjectiveShortStress`].
    ///
    /// The following conversion is attempted: a/a --- a, b/b --- b, a/a′ --- a′, b/b′ --- b′.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::{AdjectiveShortStress, AdjectiveStress};
    ///
    /// assert_eq!(AdjectiveStress::A_A.try_abbr(), Some(AdjectiveShortStress::A));
    /// assert_eq!(AdjectiveStress::B_B.try_abbr(), Some(AdjectiveShortStress::B));
    /// assert_eq!(AdjectiveStress::A_Ap.try_abbr(), Some(AdjectiveShortStress::Ap));
    /// assert_eq!(AdjectiveStress::B_Bp.try_abbr(), Some(AdjectiveShortStress::Bp));
    ///
    /// assert_eq!(AdjectiveStress::A_B.try_abbr(), None);
    /// assert_eq!(AdjectiveStress::B_Ap.try_abbr(), None);
    /// assert_eq!(AdjectiveStress::A_Cpp.try_abbr(), None);
    /// ```
    #[must_use]
    pub const fn try_abbr(self) -> Option<AdjectiveShortStress> {
        match self {
            Self::A_A | Self::B_B | Self::A_Ap | Self::B_Bp => Some(self.short),
            _ => None,
        }
    }
    /// Tries to abbreviate this dual adjective stress into [`AnyDualStress`].
    ///
    /// The following conversion is attempted: a/a --- a, b/b --- b, a/a′ --- a′, b/b′ --- b′.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::{AdjectiveStress, AnyStress};
    ///
    /// assert_eq!(AdjectiveStress::A_A.abbr(), AnyStress::A.into());
    /// assert_eq!(AdjectiveStress::B_B.abbr(), AnyStress::B.into());
    /// assert_eq!(AdjectiveStress::A_Ap.abbr(), AnyStress::Ap.into());
    /// assert_eq!(AdjectiveStress::B_Bp.abbr(), AnyStress::Bp.into());
    ///
    /// assert_eq!(AdjectiveStress::A_B.abbr(), (AnyStress::A, AnyStress::B).into());
    /// assert_eq!(AdjectiveStress::B_Ap.abbr(), (AnyStress::B, AnyStress::Ap).into());
    /// assert_eq!(AdjectiveStress::A_Cpp.abbr(), (AnyStress::A, AnyStress::Cpp).into());
    /// ```
    #[must_use]
    pub const fn abbr(self) -> AnyDualStress {
        if let Some(abbr) = self.try_abbr() { abbr.into() } else { self.into() }
    }
}

impl VerbStress {
    /// Tries to abbreviate this dual verb stress into [`VerbPresentStress`].
    ///
    /// If past form stress is a, returns the present form stress; otherwise, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::{VerbPresentStress, VerbStress};
    ///
    /// assert_eq!(VerbStress::A_A.try_abbr(), Some(VerbPresentStress::A));
    /// assert_eq!(VerbStress::B_A.try_abbr(), Some(VerbPresentStress::B));
    /// assert_eq!(VerbStress::C_A.try_abbr(), Some(VerbPresentStress::C));
    /// assert_eq!(VerbStress::Cp_A.try_abbr(), Some(VerbPresentStress::Cp));
    ///
    /// assert_eq!(VerbStress::B_B.try_abbr(), None);
    /// assert_eq!(VerbStress::C_Cp.try_abbr(), None);
    /// assert_eq!(VerbStress::Cp_Cpp.try_abbr(), None);
    /// ```
    #[must_use]
    pub const fn try_abbr(self) -> Option<VerbPresentStress> {
        if self.past == VerbPastStress::A { Some(self.present) } else { None }
    }
    /// Tries to abbreviate this dual verb stress into [`AnyDualStress`].
    ///
    /// If past form stress is a, it is removed from the dual stress.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::{AnyStress, VerbStress};
    ///
    /// assert_eq!(VerbStress::A_A.abbr(), AnyStress::A.into());
    /// assert_eq!(VerbStress::B_A.abbr(), AnyStress::B.into());
    /// assert_eq!(VerbStress::C_A.abbr(), AnyStress::C.into());
    /// assert_eq!(VerbStress::Cp_A.abbr(), AnyStress::Cp.into());
    ///
    /// assert_eq!(VerbStress::B_B.abbr(), (AnyStress::B, AnyStress::B).into());
    /// assert_eq!(VerbStress::C_Cp.abbr(), (AnyStress::C, AnyStress::Cp).into());
    /// assert_eq!(VerbStress::Cp_Cpp.abbr(), (AnyStress::Cp, AnyStress::Cpp).into());
    /// ```
    #[must_use]
    pub const fn abbr(self) -> AnyDualStress {
        if let Some(abbr) = self.try_abbr() { abbr.into() } else { self.into() }
    }
}

impl NounStress {
    /// Returns `true` if the noun's stem should be stressed.
    #[must_use]
    pub const fn is_stem_stressed(self, info: DeclInfo) -> bool {
        // Note: `is_nom_or_acc_inan` is called only when number is plural, i.e. when the
        //   accusative case always maps to either nominative or genitive depending on animacy.
        // (see declension::endings_tables::NOUN_LOOKUP, 'acc pl' section)

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
    /// Returns `true` if the noun's ending should be stressed.
    #[must_use]
    pub const fn is_ending_stressed(self, info: DeclInfo) -> bool {
        !self.is_stem_stressed(info)
    }
}

impl PronounStress {
    /// Returns `true` if the pronoun's stem should be stressed.
    #[must_use]
    pub const fn is_stem_stressed(self, info: DeclInfo) -> bool {
        match self {
            Self::A => true,
            Self::B => false,
            Self::F => info.is_plural() && info.case.is_nom_or_acc_inan(info),
        }
    }
    /// Returns `true` if the pronoun's ending should be stressed.
    #[must_use]
    pub const fn is_ending_stressed(self, info: DeclInfo) -> bool {
        !self.is_stem_stressed(info)
    }
}

impl AdjectiveFullStress {
    /// Returns `true` if the adjective's full form's stem should be stressed.
    #[must_use]
    pub const fn is_stem_stressed(self) -> bool {
        self == Self::A
    }
    /// Returns `true` if the adjective's full form's ending should be stressed.
    #[must_use]
    pub const fn is_ending_stressed(self) -> bool {
        !self.is_stem_stressed()
    }
}
impl AdjectiveShortStress {
    /// Returns `true` if the adjective's short form's stem should be stressed.
    #[must_use]
    pub const fn is_stem_stressed(self, number: Number, gender: Gender) -> Option<bool> {
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
    /// Returns `true` if the adjective's short form's ending should be stressed.
    #[must_use]
    pub const fn is_ending_stressed(self, number: Number, gender: Gender) -> Option<bool> {
        self.is_stem_stressed(number, gender).map(<bool as std::ops::Not>::not)
    }
}

impl VerbPresentStress {
    /// Returns `true` if the verb's present tense form's stem should be stressed.
    #[must_use]
    pub const fn is_stem_stressed(self, number: Number, person: Person) -> bool {
        match self {
            Self::A => true,
            Self::B => false,
            Self::C => number.is_plural() || !person.is_first(),
            Self::Cp => number.is_singular() && !person.is_first(),
        }
    }
    /// Returns `true` if the verb's present tense form's ending should be stressed.
    #[must_use]
    pub const fn is_ending_stressed(self, number: Number, person: Person) -> bool {
        !self.is_stem_stressed(number, person)
    }
}
impl VerbPastStress {
    /// Returns `true` if the verb's past tense form's stem should be stressed.
    #[must_use]
    pub const fn is_stem_stressed(self, number: Number, gender: Gender) -> Option<bool> {
        match self {
            Self::A => Some(true),
            Self::B => Some(false),
            Self::C => Some(number.is_plural() || gender != Gender::Feminine),
            Self::Cp => match (number, gender) {
                (Number::Plural, _) => Some(true),
                (_, Gender::Masculine) => Some(true),
                (_, Gender::Neuter) => Some(false),
                (_, Gender::Feminine) => Some(false),
            },
            Self::Cpp => match (number, gender) {
                (Number::Plural, _) => None,
                (_, Gender::Masculine) => Some(true), // Note: not accounting for dated -ся́ ending
                (_, Gender::Neuter) => None,
                (_, Gender::Feminine) => Some(false),
            },
        }
    }
    /// Returns `true` if the verb's past tense form's ending should be stressed.
    #[must_use]
    pub const fn is_ending_stressed(self, number: Number, gender: Gender) -> Option<bool> {
        self.is_stem_stressed(number, gender).map(<bool as std::ops::Not>::not)
    }
}
