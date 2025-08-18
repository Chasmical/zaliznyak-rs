use crate::categories::{
    Animacy, Case, Gender, Number,
    traits::{IntoAnimacy, IntoCase, IntoGender, IntoNumber},
};

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub struct Info {
    pub case: Case,
    pub number: Number,
    pub gender: Gender,
    pub animacy: Animacy,
}

impl const IntoCase for Info {
    fn case(&self) -> Case {
        self.case
    }
}
impl const IntoNumber for Info {
    fn number(&self) -> Number {
        self.number
    }
}
impl const IntoGender for Info {
    fn gender(&self) -> Gender {
        self.gender
    }
}
impl const IntoAnimacy for Info {
    fn animacy(&self) -> Animacy {
        self.animacy
    }
}
