use crate::{
    declension::{
        AdjectiveDeclension, AnyStemType, Declension, DeclensionFlags, NounDeclension,
        PronounDeclension,
    },
    stress::{AnyDualStress, DUAL_STRESS_MAX_LEN},
    util::UnsafeBuf,
};

// Longest form: °*①②③, ё (16 bytes, 8 chars)
pub const DECLENSION_FLAGS_MAX_LEN: usize = 16;

impl DeclensionFlags {
    #[inline]
    pub(crate) const fn fmt_leading_to_buf(self, dst: &mut UnsafeBuf) {
        if self.has_circle() {
            dst.push('°');
        }
        if self.has_star() {
            dst.push('*');
        }
    }
    #[inline]
    pub(crate) const fn fmt_trailing_to_buf(self, dst: &mut UnsafeBuf) {
        if self.has_any_trailing_flags() {
            if self.has_circled_one() {
                dst.push('①');
            }
            if self.has_circled_two() {
                dst.push('②');
            }
            if self.has_circled_three() {
                dst.push('③');
            }
            if self.has_alternating_yo() {
                dst.push_str(", ё");
            }
        }
    }
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_FLAGS_MAX_LEN]) -> &mut str {
        let mut dst = UnsafeBuf::new(dst);
        self.fmt_leading_to_buf(&mut dst);
        self.fmt_trailing_to_buf(&mut dst);
        dst.finish()
    }
}

// Longest form (w/ prefix): п 7°*f″/f″①②③, ё (29 bytes, 16 chars)
pub const DECLENSION_MAX_LEN: usize = "п 7".len() + DECLENSION_FLAGS_MAX_LEN + DUAL_STRESS_MAX_LEN;

const fn fmt_declension_any(
    dst: &mut [u8; DECLENSION_MAX_LEN],
    stem_type: AnyStemType,
    stress: AnyDualStress,
    flags: DeclensionFlags,
) -> &mut str {
    let mut dst = UnsafeBuf::new(dst);

    dst.push(stem_type.to_ascii_digit() as char);

    flags.fmt_leading_to_buf(&mut dst);

    let stress_len = stress.fmt_to(dst.chunk()).len();
    dst.forward(stress_len);

    flags.fmt_trailing_to_buf(&mut dst);

    dst.finish()
}

impl NounDeclension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &mut str {
        fmt_declension_any(dst, self.stem_type.into(), self.stress.into(), self.flags)
    }
}
impl PronounDeclension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &mut str {
        fmt_declension_any(dst, self.stem_type.into(), self.stress.into(), self.flags)
    }
}
impl AdjectiveDeclension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &mut str {
        fmt_declension_any(dst, self.stem_type.into(), self.stress.abbr(), self.flags)
    }
}
impl Declension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &mut str {
        let mut dst = UnsafeBuf::new(dst);

        let (stem_type, stress, flags) = match self {
            Self::Noun(decl) => {
                // no prefix for nouns
                (decl.stem_type.into(), decl.stress.into(), decl.flags)
            },
            Self::Pronoun(decl) => {
                dst.push_str("мс ");
                (decl.stem_type.into(), decl.stress.into(), decl.flags)
            },
            Self::Adjective(decl) => {
                dst.push_str("п ");
                (decl.stem_type.into(), decl.stress.abbr(), decl.flags)
            },
        };

        let len = fmt_declension_any(dst.chunk(), stem_type, stress, flags).len();
        dst.forward(len);

        dst.finish()
    }
}

impl std::fmt::Display for DeclensionFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; _]).fmt(f)
    }
}
impl std::fmt::Display for NounDeclension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; _]).fmt(f)
    }
}
impl std::fmt::Display for PronounDeclension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; _]).fmt(f)
    }
}
impl std::fmt::Display for AdjectiveDeclension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; _]).fmt(f)
    }
}
impl std::fmt::Display for Declension {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_to(&mut [0; _]).fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::{DeclensionFlags as DF, *};
    use crate::{
        declension::{AdjectiveStemType, NounStemType, PronounStemType},
        stress::{AdjectiveStress, NounStress, PronounStress},
    };

    #[test]
    fn fmt_flags() {
        assert_eq!(DF::empty().to_string(), "");
        assert_eq!(DF::STAR.to_string(), "*");
        assert_eq!(DF::CIRCLE.to_string(), "°");
        assert_eq!(DF::CIRCLED_ONE.to_string(), "①");
        assert_eq!(DF::CIRCLED_TWO.to_string(), "②");
        assert_eq!(DF::CIRCLED_THREE.to_string(), "③");
        assert_eq!(DF::ALTERNATING_YO.to_string(), ", ё");
        assert_eq!((DF::CIRCLE | DF::STAR).to_string(), "°*");
        assert_eq!((DF::CIRCLE | DF::CIRCLED_ONE | DF::ALTERNATING_YO).to_string(), "°①, ё");
        assert_eq!(DF::all().to_string(), "°*①②③, ё");
    }

    #[test]
    fn fmt_noun() {
        use {NounStemType::*, NounStress::*};

        let assert_fmt = |stem_type, stress, flags, expected: &str| {
            assert_eq!(NounDeclension { stem_type, stress, flags }.to_string(), expected);
        };

        assert_fmt(Type1, A, DF::empty(), "1a");
        assert_fmt(Type2, Bp, DF::empty(), "2b′");
        assert_fmt(Type3, F, DF::CIRCLE, "3°f");
        assert_fmt(Type4, Fp, DF::STAR, "4*f′");
        assert_fmt(Type5, Fpp, DF::CIRCLE | DF::STAR, "5°*f″");
        assert_fmt(Type6, C, DF::STAR | DF::ALTERNATING_YO, "6*c, ё");
        assert_fmt(Type7, Dp, DF::STAR | DF::CIRCLED_ONE | DF::ALTERNATING_YO, "7*d′①, ё");
        assert_fmt(Type8, E, DF::CIRCLED_ONE | DF::CIRCLED_TWO | DF::CIRCLED_THREE, "8e①②③");
        assert_fmt(Type8, Fpp, DF::all(), "8°*f″①②③, ё");
    }

    #[test]
    fn fmt_pronoun() {
        use {PronounStemType::*, PronounStress::*};

        let assert_fmt = |stem_type, stress, flags, expected: &str| {
            assert_eq!(PronounDeclension { stem_type, stress, flags }.to_string(), expected);
        };

        assert_fmt(Type1, A, DF::empty(), "1a");
        assert_fmt(Type2, B, DF::STAR, "2*b");
        assert_fmt(Type4, F, DF::STAR | DF::CIRCLED_ONE | DF::CIRCLED_TWO, "4*f①②");
        assert_fmt(Type6, B, DF::CIRCLE | DF::STAR | DF::ALTERNATING_YO, "6°*b, ё");
        assert_fmt(Type6, F, DF::all(), "6°*f①②③, ё");
    }

    #[test]
    fn fmt_adjective() {
        use {AdjectiveStemType::*, AdjectiveStress as S};

        let assert_fmt = |stem_type, stress, flags, expected: &str| {
            assert_eq!(AdjectiveDeclension { stem_type, stress, flags }.to_string(), expected);
        };

        assert_fmt(Type1, S::A_A, DF::empty(), "1a");
        assert_fmt(Type2, S::A_B, DF::empty(), "2a/b");
        assert_fmt(Type3, S::B_B, DF::STAR, "3*b");
        assert_fmt(Type4, S::B_C, DF::CIRCLE, "4°b/c");
        assert_fmt(Type5, S::A_Ap, DF::CIRCLED_ONE | DF::CIRCLED_THREE, "5a′①③");
        assert_fmt(Type6, S::B_Bp, DF::CIRCLE | DF::STAR | DF::ALTERNATING_YO, "6°*b′, ё");
        assert_fmt(Type7, S::B_Cp, DF::CIRCLED_TWO | DF::CIRCLED_THREE, "7b/c′②③");
        assert_fmt(Type1, S::A_Cpp, DF::STAR | DF::CIRCLED_ONE, "1*a/c″①");
        assert_fmt(Type7, S::B_Cpp, DF::all(), "7°*b/c″①②③, ё");
    }
}
