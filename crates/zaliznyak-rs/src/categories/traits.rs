use crate::categories::{Animacy, Case, CaseEx, Gender, GenderEx, Number};

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
        matches!(self.animacy(), Animacy::Inanimate)
    }
    fn is_animate(&self) -> bool {
        matches!(self.animacy(), Animacy::Animate)
    }
}
pub const trait IntoNumber {
    fn number(&self) -> Number;

    fn is_singular(&self) -> bool {
        matches!(self.number(), Number::Singular)
    }
    fn is_plural(&self) -> bool {
        matches!(self.number(), Number::Plural)
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
