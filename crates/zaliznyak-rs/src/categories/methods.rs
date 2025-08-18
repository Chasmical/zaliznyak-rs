use crate::categories::{Animacy, Case, CaseEx, Gender, GenderEx, Number, traits::IntoAnimacy};

impl CaseEx {
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
    pub const fn normalize(self) -> Gender {
        self.try_into().unwrap_or(Gender::Feminine)
    }
}

impl Case {
    pub const fn acc_is_gen<A>(self, animacy: A) -> Option<bool>
    where A: [const] IntoAnimacy + [const] std::marker::Destruct {
        match self {
            Self::Nominative => Some(false),
            Self::Genitive => Some(true),
            Self::Accusative => Some(animacy.is_animate()),
            _ => None,
        }
    }

    pub const fn is_nom_or_acc_inan<A>(self, info: A) -> bool
    where A: [const] IntoAnimacy + [const] std::marker::Destruct {
        self == Self::Nominative || self == Self::Accusative && info.is_inanimate()
    }
    pub const fn is_gen_or_acc_an<A>(self, info: A) -> bool
    where A: [const] IntoAnimacy + [const] std::marker::Destruct {
        self == Self::Genitive || self == Self::Accusative && info.is_animate()
    }
}
impl Animacy {
    pub const fn acc_case(self) -> Case {
        match self {
            Self::Inanimate => Case::Nominative,
            Self::Animate => Case::Genitive,
        }
    }
}
