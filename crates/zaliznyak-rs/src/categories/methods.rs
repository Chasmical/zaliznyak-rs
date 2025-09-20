use crate::categories::{Animacy, Case, CaseEx, Gender, GenderEx, Number, traits::IntoAnimacy};

impl CaseEx {
    /// Normalizes this case, converting secondary cases into primary cases.
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn normalize_with(self, number: Number) -> (Case, Number) {
        match self {
            Self::Partitive => (Case::Genitive, number),
            Self::Translative => (Case::Nominative, Number::Plural),
            Self::Locative => (Case::Prepositional, number),
            _ => (unsafe { std::mem::transmute::<CaseEx, Case>(self) }, number),
        }
    }
}
impl GenderEx {
    /// Normalizes this gender, converting [`GenderEx::Common`] to [`Feminine`][Gender::Feminine].
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub const fn normalize(self) -> Gender {
        self.try_into().unwrap_or(Gender::Feminine)
    }
}

impl Case {
    /// Determines whether the [`Accusative`] case's default form for specified animacy
    ///   is [`Nominative`] or [`Genitive`], returning the following values:
    ///
    /// | Case             | Animacy       | Result        |
    /// |------------------|---------------|---------------|
    /// | [`Nominative`]   | (any)         | `Some(false)` |
    /// | [`Genitive`]     | (any)         | `Some(true)`  |
    /// | [`Accusative`]   | [`Inanimate`] | `Some(false)` |
    /// | [`Accusative`]   | [`Animate`]   | `Some(true)`  |
    /// | (any other case) | (any)         | `None`        |
    ///
    /// [`Nominative`]: Case::Nominative
    /// [`Genitive`]: Case::Genitive
    /// [`Accusative`]: Case::Accusative
    /// [`Inanimate`]: Animacy::Inanimate
    /// [`Animate`]: Animacy::Animate
    #[must_use]
    pub const fn acc_is_gen<A>(self, animacy: A) -> Option<bool>
    where A: [const] IntoAnimacy + [const] std::marker::Destruct {
        match self {
            Self::Nominative => Some(false),
            Self::Genitive => Some(true),
            Self::Accusative => Some(animacy.is_animate()),
            _ => None,
        }
    }

    /// Returns `true` if either:
    /// a) case is [`Nominative`][Case::Nominative], or
    /// b) case is [`Accusative`][Case::Accusative] and animacy is [`Inanimate`][Animacy::Inanimate].
    #[must_use]
    pub const fn is_nom_or_acc_inan<A>(self, info: A) -> bool
    where A: [const] IntoAnimacy + [const] std::marker::Destruct {
        self == Self::Nominative || self == Self::Accusative && info.is_inanimate()
    }
    /// Returns `true` if either:
    /// a) case is [`Genitive`][Case::Genitive], or
    /// b) case is [`Accusative`][Case::Accusative] and animacy is [`Animate`][Animacy::Animate].
    #[must_use]
    pub const fn is_gen_or_acc_an<A>(self, info: A) -> bool
    where A: [const] IntoAnimacy + [const] std::marker::Destruct {
        self == Self::Genitive || self == Self::Accusative && info.is_animate()
    }
}
impl Animacy {
    /// Returns the [`Accusative`][Case::Accusative] case's default form for this animacy
    /// ([`Nominative`][Case::Nominative] or [`Genitive`][Case::Genitive]).
    #[must_use]
    pub const fn acc_case(self) -> Case {
        match self {
            Self::Inanimate => Case::Nominative,
            Self::Animate => Case::Genitive,
        }
    }
}
