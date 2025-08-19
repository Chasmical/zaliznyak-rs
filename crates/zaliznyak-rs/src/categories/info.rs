use crate::{
    categories::{
        Animacy, Case, Gender, Number, Person, Tense,
        traits::{IntoAnimacy, IntoCase, IntoGender, IntoNumber, IntoPerson, IntoTense},
    },
    util::UnsafeParser,
};

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub struct DeclInfo {
    pub case: Case,
    pub number: Number,
    pub gender: Gender,
    pub animacy: Animacy,
}

impl const IntoCase for DeclInfo {
    fn case(&self) -> Case {
        self.case
    }
}
impl const IntoNumber for DeclInfo {
    fn number(&self) -> Number {
        self.number
    }
}
impl const IntoGender for DeclInfo {
    fn gender(&self) -> Gender {
        self.gender
    }
}
impl const IntoAnimacy for DeclInfo {
    fn animacy(&self) -> Animacy {
        self.animacy
    }
}

#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub struct ConjInfo {
    pub tense: Tense,
    pub number: Number,
    pub gender: Gender,
    pub person: Person,
}

impl const IntoTense for ConjInfo {
    fn tense(&self) -> Tense {
        self.tense
    }
}
impl const IntoNumber for ConjInfo {
    fn number(&self) -> Number {
        self.number
    }
}
impl const IntoGender for ConjInfo {
    fn gender(&self) -> Gender {
        self.gender
    }
}
impl const IntoPerson for ConjInfo {
    fn person(&self) -> Person {
        self.person
    }
}

impl std::str::FromStr for DeclInfo {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = UnsafeParser::new(s);

        let (mut case, mut number, mut gender, mut animacy) = (None, None, None, None);

        enum Part {
            Case(Case),
            Number(Number),
            Gender(Gender),
            Animacy(Animacy),
        }

        while !parser.finished() {
            let part = match parser.read_char() {
                Some('И') => Part::Case(Case::Nominative),
                Some('Р') => Part::Case(Case::Genitive),
                Some('Д') => Part::Case(Case::Dative),
                Some('В') => Part::Case(Case::Accusative),
                Some('Т') => Part::Case(Case::Instrumental),
                Some('П') => Part::Case(Case::Prepositional),
                Some('е') if parser.skip('д') => Part::Number(Number::Singular),
                Some('м') if parser.skip('н') => Part::Number(Number::Plural),
                Some('м') => Part::Gender(Gender::Masculine),
                Some('с') => Part::Gender(Gender::Neuter),
                Some('ж') => Part::Gender(Gender::Feminine),
                Some('о') if parser.skip_str("душ") => Part::Animacy(Animacy::Animate),
                Some('н') if parser.skip_str("еод") => Part::Animacy(Animacy::Inanimate),
                _ => return Err(()),
            };
            _ = parser.skip('.');
            _ = parser.skip(' ');

            match part {
                Part::Case(x) => {
                    case = Some(x);
                    if parser.skip('п') {
                        _ = parser.skip('.');
                        _ = parser.skip(' ');
                    }
                },
                Part::Number(x) => {
                    number = Some(x);
                    if parser.skip('ч') {
                        _ = parser.skip('.');
                        _ = parser.skip(' ');
                    }
                },
                Part::Gender(x) => {
                    gender = Some(x);
                    if parser.skip('р') {
                        _ = parser.skip('.');
                        _ = parser.skip(' ');
                    }
                },
                Part::Animacy(x) => animacy = Some(x),
            };
        }

        Ok(Self {
            case: case.unwrap_or_default(),
            number: number.unwrap_or_default(),
            gender: gender.unwrap_or_default(),
            animacy: animacy.unwrap_or_default(),
        })
    }
}
