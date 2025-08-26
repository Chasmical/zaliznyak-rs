use crate::{
    alphabet::utf8,
    categories::{Animacy, Gender, GenderEx, Number},
    declension::{
        AdjectiveDeclension, Declension, DeclensionKind, NounDeclension, ParseDeclensionError,
    },
    noun::NounInfo,
    util::{PartialFromStr, UnsafeParser},
};
use thiserror::Error;

#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum ParseNounInfoError {
    #[error("invalid character in place of gender or type")]
    InvalidGenderOrType,
    #[error("error parsing declension: {0}")]
    InvalidDeclension(ParseDeclensionError),
    #[error("different animacy was specified in two places")]
    InconsistentAnimacy,
    #[error("animacy was not specified for plurale tantum")]
    NoAnimacy,
    #[error("both plurale and singulare tantums were specified")]
    BothTantums,
    #[error("invalid format")]
    Invalid,
}

impl const PartialFromStr for NounInfo {
    fn partial_from_str(parser: &mut UnsafeParser) -> Result<Self, Self::Err> {
        let mut tantum = None;
        let mut animacy = Some(Animacy::Inanimate);

        let gender = match parser.peek::<2>() {
            Some(&utf8::М) => {
                parser.forward(2);
                // Handle 'мо-жо' and 'мн.' cases
                match parser.peek::<2>() {
                    Some(&utf8::О) => {
                        parser.forward(2);
                        animacy = Some(Animacy::Animate);

                        // 'мо-жо', common gender
                        if parser.skip_str("-жо") {
                            GenderEx::Common
                        } else {
                            GenderEx::Masculine
                        }
                    },
                    Some(&utf8::Н) => {
                        // 'мн.', plurale tantum
                        parser.forward(2);
                        tantum = Some(Number::Plural);

                        if !parser.skip('.') {
                            return Err(Self::Err::Invalid);
                        }

                        // Explicitly specified animacy
                        if parser.skip_str(" неод.") {
                            animacy = Some(Animacy::Inanimate);
                        } else if parser.skip_str(" одуш.") {
                            animacy = Some(Animacy::Animate);
                        } else {
                            animacy = None;
                        }

                        Default::default()
                    },
                    // 'м', masculine inanimate
                    _ => GenderEx::Masculine,
                }
            },
            // 'с' or 'со', neuter gender
            Some(&utf8::С) => {
                parser.forward(2);
                if parser.skip('о') {
                    animacy = Some(Animacy::Animate);
                }
                GenderEx::Neuter
            },
            // 'ж' or 'жо', feminine gender
            Some(&utf8::Ж) => {
                parser.forward(2);
                if parser.skip('о') {
                    animacy = Some(Animacy::Animate);
                }
                GenderEx::Feminine
            },
            _ => return Err(Self::Err::InvalidGenderOrType),
        };

        // Expect a space between gender/animacy and declension
        if !parser.skip(' ') {
            return Err(Self::Err::Invalid);
        }

        let declension;
        let mut declension_gender = gender.normalize();

        if parser.skip('0') {
            declension = None;
            // Don't expect anything else after 0
        } else {
            let kind;
            let in_brackets = parser.skip('<');

            // Expect unusual declension in brackets (diff gender or adjective)
            if in_brackets {
                match parser.peek::<2>() {
                    Some(&utf8::П) => {
                        // Adjective declension
                        parser.forward(2);
                        kind = DeclensionKind::Adjective;
                    },
                    Some(gender_char) => {
                        // Different gender declension
                        parser.forward(2);
                        kind = DeclensionKind::Noun;

                        declension_gender = match gender_char {
                            &utf8::М => Gender::Masculine,
                            &utf8::С => Gender::Neuter,
                            &utf8::Ж => Gender::Feminine,
                            _ => return Err(Self::Err::InvalidGenderOrType),
                        };
                        let declension_animacy =
                            if parser.skip('о') { Animacy::Animate } else { Animacy::Inanimate };

                        // Animacy must be the same though
                        if let Some(an) = animacy
                            && declension_animacy != an
                        {
                            return Err(Self::Err::InconsistentAnimacy);
                        }
                        animacy = Some(declension_animacy);
                    },
                    None => return Err(Self::Err::InvalidGenderOrType),
                };

                // Expect another space between declension type/gender and declension
                if !parser.skip(' ') {
                    return Err(Self::Err::Invalid);
                }
            } else {
                kind = DeclensionKind::Noun;
            }

            // Parse declension of detected type
            declension = Some(match kind {
                DeclensionKind::Noun => Declension::Noun(
                    NounDeclension::partial_from_str(parser)
                        .map_err(Self::Err::InvalidDeclension)?,
                ),
                DeclensionKind::Adjective => Declension::Adjective(
                    AdjectiveDeclension::partial_from_str(parser)
                        .map_err(Self::Err::InvalidDeclension)?,
                ),
                _ => unreachable!(),
            });

            // Parse '—' singulare tantum mark
            if parser.skip('—') {
                if tantum.is_some() {
                    return Err(Self::Err::BothTantums);
                }
                tantum = Some(Number::Singular);
            }

            // Close brackets
            if in_brackets && !parser.skip('>') {
                return Err(Self::Err::Invalid);
            }
        }

        Ok(NounInfo {
            gender,
            declension_gender,
            declension,
            animacy: animacy.ok_or(Self::Err::NoAnimacy)?,
            tantum,
        })
    }
}

impl std::str::FromStr for NounInfo {
    type Err = ParseNounInfoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_or_err(s, Self::Err::Invalid)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        declension::{AdjectiveStemType, DeclensionFlags, NounStemType},
        stress::{AdjectiveStress, NounStress},
    };

    use super::*;

    #[test]
    fn parse() {
        // Some uncomplicated nouns
        assert_eq!(
            "мо 3b".parse(),
            Ok(NounInfo {
                gender: GenderEx::Masculine,
                declension_gender: Gender::Masculine,
                animacy: Animacy::Animate,
                tantum: None,
                declension: Some(Declension::Noun(NounDeclension {
                    stem_type: NounStemType::Type3,
                    stress: NounStress::B,
                    flags: DeclensionFlags::empty(),
                })),
            }),
        );
        assert_eq!(
            "с 4a①—".parse(),
            Ok(NounInfo {
                gender: GenderEx::Neuter,
                declension_gender: Gender::Neuter,
                animacy: Animacy::Inanimate,
                tantum: Some(Number::Singular),
                declension: Some(Declension::Noun(NounDeclension {
                    stem_type: NounStemType::Type4,
                    stress: NounStress::A,
                    flags: DeclensionFlags::CIRCLED_ONE,
                })),
            }),
        );
        assert_eq!(
            "со <жо 6*f>".parse(),
            Ok(NounInfo {
                gender: GenderEx::Neuter,
                declension_gender: Gender::Feminine,
                animacy: Animacy::Animate,
                tantum: None,
                declension: Some(Declension::Noun(NounDeclension {
                    stem_type: NounStemType::Type6,
                    stress: NounStress::F,
                    flags: DeclensionFlags::STAR,
                })),
            }),
        );

        // Common gender and plurale tantum
        assert_eq!(
            "мо-жо 5c①".parse(),
            Ok(NounInfo {
                gender: GenderEx::Common,
                declension_gender: Gender::Feminine,
                animacy: Animacy::Animate,
                tantum: None,
                declension: Some(Declension::Noun(NounDeclension {
                    stem_type: NounStemType::Type5,
                    stress: NounStress::C,
                    flags: DeclensionFlags::CIRCLED_ONE,
                })),
            }),
        );
        assert_eq!(
            "мн. <мо 4a>".parse(),
            Ok(NounInfo {
                gender: GenderEx::Masculine,
                declension_gender: Gender::Masculine,
                animacy: Animacy::Animate,
                tantum: Some(Number::Plural),
                declension: Some(Declension::Noun(NounDeclension {
                    stem_type: NounStemType::Type4,
                    stress: NounStress::A,
                    flags: DeclensionFlags::empty(),
                })),
            }),
        );

        // Plurale tantum with animacy explicitly specified
        assert_eq!(
            "мн. одуш. 0".parse(),
            Ok(NounInfo {
                gender: GenderEx::Masculine,
                declension_gender: Gender::Masculine,
                animacy: Animacy::Animate,
                tantum: Some(Number::Plural),
                declension: None,
            }),
        );
        assert_eq!(
            "мн. неод. <п 4a>".parse(),
            Ok(NounInfo {
                gender: GenderEx::Masculine,
                declension_gender: Gender::Masculine,
                animacy: Animacy::Inanimate,
                tantum: Some(Number::Plural),
                declension: Some(Declension::Adjective(AdjectiveDeclension {
                    stem_type: AdjectiveStemType::Type4,
                    stress: AdjectiveStress::A,
                    flags: DeclensionFlags::empty(),
                })),
            }),
        );

        assert_eq!("мн. 0".parse::<NounInfo>(), Err(ParseNounInfoError::NoAnimacy));
        assert_eq!("мн. <п 4a>".parse::<NounInfo>(), Err(ParseNounInfoError::NoAnimacy));

        // Adjective declension
        assert_eq!(
            "со <п 3b—>".parse(),
            Ok(NounInfo {
                gender: GenderEx::Neuter,
                declension_gender: Gender::Neuter,
                animacy: Animacy::Animate,
                tantum: Some(Number::Singular),
                declension: Some(Declension::Adjective(AdjectiveDeclension {
                    stem_type: AdjectiveStemType::Type3,
                    stress: AdjectiveStress::B,
                    flags: DeclensionFlags::empty(),
                })),
            }),
        );
    }
}
