use crate::categories::{Animacy, Case, CaseEx, Gender, GenderEx, Number, Person, Tense};

/// Conversion into a [`CaseEx`].
pub const trait IntoCaseEx {
    /// Gets this object's grammatical case.
    #[must_use]
    fn case_ex(&self) -> CaseEx;
}
/// Conversion into a [`Case`].
pub const trait IntoCase {
    /// Gets this object's grammatical case.
    #[must_use]
    fn case(&self) -> Case;
}
/// Conversion into a [`GenderEx`].
pub const trait IntoGenderEx {
    /// Gets this object's grammatical gender.
    #[must_use]
    fn gender_ex(&self) -> GenderEx;
}
/// Conversion into a [`Gender`].
pub const trait IntoGender {
    /// Gets this object's grammatical gender.
    #[must_use]
    fn gender(&self) -> Gender;
}

/// Conversion into an [`Animacy`].
pub const trait IntoAnimacy {
    /// Gets this object's grammatical animacy.
    #[must_use]
    fn animacy(&self) -> Animacy;

    /// Returns `true` if the animacy is [`Inanimate`][Animacy::Inanimate].
    #[must_use]
    fn is_inanimate(&self) -> bool {
        self.animacy() == Animacy::Inanimate
    }
    /// Returns `true` if the animacy is [`Animate`][Animacy::Animate].
    #[must_use]
    fn is_animate(&self) -> bool {
        self.animacy() == Animacy::Animate
    }
}
/// Conversion into a [`Number`].
pub const trait IntoNumber {
    /// Gets this object's grammatical number.
    #[must_use]
    fn number(&self) -> Number;

    /// Returns `true` if the number is [`Singular`][Number::Singular].
    #[must_use]
    fn is_singular(&self) -> bool {
        self.number() == Number::Singular
    }
    /// Returns `true` if the number is [`Plural`][Number::Plural].
    #[must_use]
    fn is_plural(&self) -> bool {
        self.number() == Number::Plural
    }
}

/// Conversion into a [`Tense`].
pub const trait IntoTense {
    /// Gets this object's grammatical tense.
    #[must_use]
    fn tense(&self) -> Tense;

    /// Returns `true` if the tense is [`Present`][Tense::Present].
    #[must_use]
    fn is_present(&self) -> bool {
        self.tense() == Tense::Present
    }
    /// Returns `true` if the tense is [`Past`][Tense::Past].
    #[must_use]
    fn is_past(&self) -> bool {
        self.tense() == Tense::Past
    }
}
/// Conversion into a [`Person`].
pub const trait IntoPerson {
    /// Gets this object's grammatical person.
    #[must_use]
    fn person(&self) -> Person;

    /// Returns `true` if the person is [`First`][Person::First].
    #[must_use]
    fn is_first(&self) -> bool {
        self.person() == Person::First
    }
    /// Returns `true` if the person is [`Second`][Person::Second].
    #[must_use]
    fn is_second(&self) -> bool {
        self.person() == Person::Second
    }
    /// Returns `true` if the person is [`Third`][Person::Third].
    #[must_use]
    fn is_third(&self) -> bool {
        self.person() == Person::Third
    }
}

impl const IntoCaseEx for CaseEx {
    fn case_ex(&self) -> CaseEx {
        *self
    }
}
impl const IntoCase for Case {
    fn case(&self) -> Case {
        *self
    }
}
impl const IntoGenderEx for GenderEx {
    fn gender_ex(&self) -> GenderEx {
        *self
    }
}
impl const IntoGender for Gender {
    fn gender(&self) -> Gender {
        *self
    }
}
impl const IntoAnimacy for Animacy {
    fn animacy(&self) -> Animacy {
        *self
    }
}
impl const IntoNumber for Number {
    fn number(&self) -> Number {
        *self
    }
}
impl const IntoTense for Tense {
    fn tense(&self) -> Tense {
        *self
    }
}
impl const IntoPerson for Person {
    fn person(&self) -> Person {
        *self
    }
}

impl<T: [const] IntoCase> const IntoCaseEx for T {
    fn case_ex(&self) -> CaseEx {
        self.case().into()
    }
}
impl<T: [const] IntoGender> const IntoGenderEx for T {
    fn gender_ex(&self) -> GenderEx {
        self.gender().into()
    }
}
