use crate::{
    categories::{Animacy, Gender, GenderEx, Number},
    declension::{DECLENSION_MAX_LEN, Declension},
    noun::{Noun, NounInfo},
    util::UnsafeBuf,
};

// Longest form: мн. неод. <п 7°*f″/f″①②③, ё—> (50 bytes, 29 chars)
// Max additions: мн. неод. <п —> (+24 bytes, +15 chars)
pub const NOUN_INFO_MAX_LEN: usize = DECLENSION_MAX_LEN + 24;

impl NounInfo {
    pub const fn fmt_to<'a>(&self, dst: &'a mut [u8; NOUN_INFO_MAX_LEN]) -> &'a mut str {
        let mut dst = UnsafeBuf::new(dst);

        // Include braces if the declension is non-noun
        let mut need_braces = matches!(self.declension, Some(Declension::Adjective(_)));

        // If it's a pluralia tantum, append 'мн.'
        if self.tantum == Some(Number::Plural) {
            dst.push_str("мн.");
            need_braces = self.declension != None;

            // If gender and animacy won't be specified in braces (0 or adjective declension),
            // include animacy right after 'мн.': 'мн. неод.', 'мн. одуш.'.
            if matches!(self.declension, None | Some(Declension::Adjective(_))) {
                dst.push_str(match self.animacy {
                    Animacy::Inanimate => " неод.",
                    Animacy::Animate => " одуш.",
                });
            }
        } else {
            // Append gender and animacy, in short form
            match self.gender {
                GenderEx::Masculine => dst.push('м'),
                GenderEx::Neuter => dst.push('с'),
                GenderEx::Feminine => dst.push('ж'),
                GenderEx::Common => dst.push_str("мо-жо"),
            };
            if self.gender != GenderEx::Common && self.animacy == Animacy::Animate {
                dst.push('о');
            }
            // If declension gender doesn't match actual gender, make sure to include it in braces
            if self.declension_gender != self.gender.normalize() {
                need_braces = true;
            }
        }

        // Space between gender/animacy and declension
        dst.push(' ');

        if need_braces {
            dst.push('<');
        }

        if let Some(declension) = self.declension {
            match declension {
                Declension::Noun(decl) => {
                    // Append overridden gender and animacy, but only if in braces
                    if need_braces {
                        dst.push(match self.declension_gender {
                            Gender::Masculine => 'м',
                            Gender::Neuter => 'с',
                            Gender::Feminine => 'ж',
                        });
                        if self.animacy == Animacy::Animate {
                            dst.push('о');
                        }
                        dst.push(' ');
                    }
                    // Format the noun declension
                    let decl_len = decl.fmt_to(dst.chunk()).len();
                    dst.forward(decl_len);
                },
                Declension::Pronoun(_) => {
                    unimplemented!(); // Nouns don't decline by pronoun declension
                },
                Declension::Adjective(decl) => {
                    // Append 'п ' prefix
                    dst.push_str("п ");
                    // Format the adjective declension
                    let decl_len = decl.fmt_to(dst.chunk()).len();
                    dst.forward(decl_len);
                },
            };
        } else {
            dst.push('0');
        }

        // If it's a singularia tantum, append '—'
        if self.tantum == Some(Number::Singular) {
            dst.push('—');
        }
        if need_braces {
            dst.push('>');
        }

        dst.finish()
    }
}

impl std::fmt::Display for NounInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; _]).fmt(f)
    }
}
impl std::fmt::Display for Noun {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.stem, self.info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        declension::{
            AdjectiveDeclension, AdjectiveStemType, DeclensionFlags, NounDeclension, NounStemType,
        },
        stress::{AdjectiveStress, NounStress},
    };

    #[test]
    fn fmt() {
        // Some uncomplicated nouns
        assert_eq!(
            NounInfo {
                gender: GenderEx::Feminine,
                declension_gender: Gender::Feminine,
                animacy: Animacy::Animate,
                tantum: None,
                declension: Some(Declension::Noun(NounDeclension {
                    stem_type: NounStemType::Type4,
                    stress: NounStress::B,
                    flags: DeclensionFlags::STAR | DeclensionFlags::CIRCLED_TWO,
                })),
            }
            .to_string(),
            "жо 4*b②",
        );
        assert_eq!(
            NounInfo {
                gender: GenderEx::Neuter,
                declension_gender: Gender::Masculine,
                animacy: Animacy::Inanimate,
                tantum: None,
                declension: Some(Declension::Noun(NounDeclension {
                    stem_type: NounStemType::Type6,
                    stress: NounStress::Fp,
                    flags: DeclensionFlags::CIRCLE | DeclensionFlags::CIRCLED_ONE,
                })),
            }
            .to_string(),
            "с <м 6°f′①>",
        );

        // Common gender and tantums
        assert_eq!(
            NounInfo {
                gender: GenderEx::Common,
                declension_gender: Gender::Feminine,
                animacy: Animacy::Animate,
                tantum: None,
                declension: Some(Declension::Noun(NounDeclension {
                    stem_type: NounStemType::Type2,
                    stress: NounStress::C,
                    flags: DeclensionFlags::STAR | DeclensionFlags::ALTERNATING_YO,
                })),
            }
            .to_string(),
            "мо-жо 2*c, ё",
        );
        assert_eq!(
            NounInfo {
                gender: GenderEx::Common,
                declension_gender: Gender::Neuter,
                animacy: Animacy::Animate,
                tantum: Some(Number::Singular),
                declension: Some(Declension::Noun(NounDeclension {
                    stem_type: NounStemType::Type2,
                    stress: NounStress::A,
                    flags: DeclensionFlags::empty(),
                })),
            }
            .to_string(),
            "мо-жо <со 2a—>",
        );
        assert_eq!(
            NounInfo {
                gender: GenderEx::Masculine,
                declension_gender: Gender::Masculine,
                animacy: Animacy::Animate,
                tantum: Some(Number::Plural),
                declension: Some(Declension::Noun(NounDeclension {
                    stem_type: NounStemType::Type3,
                    stress: NounStress::A,
                    flags: DeclensionFlags::STAR,
                })),
            }
            .to_string(),
            "мн. <мо 3*a>",
        );

        // Pluralia tantums, with animacy specified
        assert_eq!(
            NounInfo {
                // Note: gender isn't used here at all
                gender: GenderEx::Neuter,
                declension_gender: Gender::Neuter,
                animacy: Animacy::Inanimate,
                tantum: Some(Number::Plural),
                declension: None,
            }
            .to_string(),
            "мн. неод. 0",
        );
        assert_eq!(
            NounInfo {
                // Note: gender isn't used here at all
                gender: GenderEx::Neuter,
                declension_gender: Gender::Neuter,
                animacy: Animacy::Inanimate,
                tantum: Some(Number::Plural),
                declension: Some(Declension::Adjective(AdjectiveDeclension {
                    stem_type: AdjectiveStemType::Type1,
                    stress: AdjectiveStress::B,
                    flags: DeclensionFlags::empty(),
                })),
            }
            .to_string(),
            "мн. неод. <п 1b>",
        );
    }
}
