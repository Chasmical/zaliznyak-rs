use crate::categories::{Animacy, Case, CaseEx, Gender, GenderEx, Number, Person, Tense};

pub const trait IntoCaseEx {
    fn case_ex(&self) -> CaseEx;
}
pub const trait IntoCase {
    fn case(&self) -> Case;
}
pub const trait IntoGenderEx {
    fn gender_ex(&self) -> GenderEx;
}
pub const trait IntoGender {
    fn gender(&self) -> Gender;
}

pub const trait IntoAnimacy {
    fn animacy(&self) -> Animacy;

    fn is_inanimate(&self) -> bool {
        self.animacy() == Animacy::Inanimate
    }
    fn is_animate(&self) -> bool {
        self.animacy() == Animacy::Animate
    }
}
pub const trait IntoNumber {
    fn number(&self) -> Number;

    fn is_singular(&self) -> bool {
        self.number() == Number::Singular
    }
    fn is_plural(&self) -> bool {
        self.number() == Number::Plural
    }
}

pub const trait IntoTense {
    fn tense(&self) -> Tense;

    fn is_present(&self) -> bool {
        self.tense() == Tense::Present
    }
    fn is_past(&self) -> bool {
        self.tense() == Tense::Past
    }
}
pub const trait IntoPerson {
    fn person(&self) -> Person;

    fn is_first(&self) -> bool {
        self.person() == Person::First
    }
    fn is_second(&self) -> bool {
        self.person() == Person::Second
    }
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
