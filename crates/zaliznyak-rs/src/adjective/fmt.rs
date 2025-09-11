use crate::{
    adjective::{AdjectiveFlags, AdjectiveInfo, AdjectiveKind},
    declension::{DECLENSION_MAX_LEN, Declension, DeclensionKind},
    util::UnsafeBuf,
};

// Longest form: числ.-п <п 7°*f″/f″①②③, ё>⌧~ (48 bytes, 28 chars)
// Max additions: числ.-п <п >⌧~ (+22 bytes, +14 chars)
pub const ADJECTIVE_INFO_MAX_LEN: usize = DECLENSION_MAX_LEN + 22;

impl AdjectiveInfo {
    pub const fn fmt_to<'a>(&self, dst: &'a mut [u8; ADJECTIVE_INFO_MAX_LEN]) -> &'a mut str {
        let mut dst = UnsafeBuf::new(dst);

        let assume_kind = match self.kind {
            AdjectiveKind::Regular => {
                dst.push_str("п ");
                DeclensionKind::Adjective
            },
            AdjectiveKind::Pronoun => {
                dst.push_str("мс-п ");
                DeclensionKind::Pronoun
            },
            AdjectiveKind::Numeral => {
                dst.push_str("числ.-п ");
                DeclensionKind::Noun
            },
        };

        if let Some(decl) = self.declension {
            let need_brackets = decl.kind() != assume_kind;
            if need_brackets {
                dst.push('<');

                match decl {
                    Declension::Adjective(_) => dst.push_str("п "),
                    Declension::Pronoun(_) => dst.push_str("мс "),
                    Declension::Noun(_) => unimplemented!(), // Adjectives don't decline by noun declension
                }
            }

            match decl {
                Declension::Adjective(decl) => {
                    let len = decl.fmt_to(dst.chunk()).len();
                    dst.forward(len);
                },
                Declension::Pronoun(decl) => {
                    let len = decl.fmt_to(dst.chunk()).len();
                    dst.forward(len);
                },
                Declension::Noun(_) => {
                    unimplemented!() // Adjectives don't decline by noun declension
                },
            }

            if need_brackets {
                dst.push('>');
            }

            if self.kind == AdjectiveKind::Regular {
                let len = self.flags.fmt_to(dst.chunk()).len();
                dst.forward(len);
            }
        }

        dst.finish()
    }
}

impl AdjectiveFlags {
    pub const fn fmt_to(self, dst: &mut [u8; 4]) -> &mut str {
        let mut dst = UnsafeBuf::new(dst);

        let short_form_flags = self.intersection(Self::BOXED_CROSS);
        if !short_form_flags.is_empty() {
            // FIXME(const-hack): Remove these consts when PartialEq on bitflags is constified.
            const MINUS: u8 = AdjectiveFlags::MINUS.bits();
            const CROSS: u8 = AdjectiveFlags::CROSS.bits();
            const BOXED_CROSS: u8 = AdjectiveFlags::BOXED_CROSS.bits();

            dst.push(match short_form_flags.bits() {
                MINUS => '—',
                CROSS => '✕',
                BOXED_CROSS => '⌧',
                _ => unreachable!(),
            });
        }

        if self.has_no_comparative_form() {
            dst.push('~');
        }

        dst.finish()
    }
}

impl std::fmt::Display for AdjectiveInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; _]).fmt(f)
    }
}
impl std::fmt::Display for AdjectiveFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; _]).fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        adjective::AdjectiveFlags,
        declension::{
            AdjectiveDeclension, AdjectiveStemType, DeclensionFlags, PronounDeclension,
            PronounStemType,
        },
        stress::{AdjectiveStress, PronounStress},
    };

    #[test]
    fn fmt() {
        // Some simple adjectives
        assert_eq!(
            AdjectiveInfo {
                kind: AdjectiveKind::Regular,
                flags: AdjectiveFlags::empty(),
                declension: Some(Declension::Adjective(AdjectiveDeclension {
                    stem_type: AdjectiveStemType::Type4,
                    stress: AdjectiveStress::A_Cp,
                    flags: DeclensionFlags::STAR,
                })),
            }
            .to_string(),
            "п 4*a/c′",
        );
        assert_eq!(
            AdjectiveInfo {
                kind: AdjectiveKind::Pronoun,
                flags: AdjectiveFlags::empty(),
                declension: Some(Declension::Pronoun(PronounDeclension {
                    stem_type: PronounStemType::Type2,
                    stress: PronounStress::B,
                    flags: DeclensionFlags::STAR,
                })),
            }
            .to_string(),
            "мс-п 2*b",
        );

        // Adjectives with different declension
        assert_eq!(
            AdjectiveInfo {
                kind: AdjectiveKind::Numeral,
                flags: AdjectiveFlags::empty(),
                declension: Some(Declension::Adjective(AdjectiveDeclension {
                    stem_type: AdjectiveStemType::Type1,
                    stress: AdjectiveStress::A,
                    flags: DeclensionFlags::empty(),
                })),
            }
            .to_string(),
            "числ.-п <п 1a>",
        );
        assert_eq!(
            AdjectiveInfo {
                kind: AdjectiveKind::Regular,
                flags: AdjectiveFlags::empty(),
                declension: Some(Declension::Pronoun(PronounDeclension {
                    stem_type: PronounStemType::Type6,
                    stress: PronounStress::F,
                    flags: DeclensionFlags::STAR,
                })),
            }
            .to_string(),
            "п <мс 6*f>",
        );

        // Adjectives with flags
        assert_eq!(
            AdjectiveInfo {
                kind: AdjectiveKind::Regular,
                flags: AdjectiveFlags::CROSS | AdjectiveFlags::NO_COMPARATIVE_FORM,
                declension: Some(Declension::Adjective(AdjectiveDeclension {
                    stem_type: AdjectiveStemType::Type3,
                    stress: AdjectiveStress::A_B,
                    flags: DeclensionFlags::empty(),
                })),
            }
            .to_string(),
            "п 3a/b✕~",
        );
        assert_eq!(
            AdjectiveInfo {
                kind: AdjectiveKind::Regular,
                flags: AdjectiveFlags::BOXED_CROSS,
                declension: Some(Declension::Adjective(AdjectiveDeclension {
                    stem_type: AdjectiveStemType::Type1,
                    stress: AdjectiveStress::B_C,
                    flags: DeclensionFlags::empty(),
                })),
            }
            .to_string(),
            "п 1b/c⌧",
        );
    }
}
