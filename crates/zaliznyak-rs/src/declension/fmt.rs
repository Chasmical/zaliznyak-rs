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
    flags: DeclensionFlags,
    stress: AnyDualStress,
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
        fmt_declension_any(dst, self.stem_type.into(), self.flags, self.stress.into())
    }
}
impl PronounDeclension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &mut str {
        fmt_declension_any(dst, self.stem_type.into(), self.flags, self.stress.into())
    }
}
impl AdjectiveDeclension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &mut str {
        fmt_declension_any(dst, self.stem_type.into(), self.flags, self.stress.into())
    }
}
impl Declension {
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_MAX_LEN]) -> &mut str {
        let mut dst = UnsafeBuf::new(dst);

        let (stem_type, flags, stress) = match self {
            Self::Noun(decl) => {
                // no prefix for nouns
                (decl.stem_type.into(), decl.flags, decl.stress.into())
            },
            Self::Pronoun(decl) => {
                dst.push_str("мс ");
                (decl.stem_type.into(), decl.flags, decl.stress.into())
            },
            Self::Adjective(decl) => {
                dst.push_str("п ");
                (decl.stem_type.into(), decl.flags, decl.stress.into())
            },
        };

        let len = fmt_declension_any(dst.chunk(), stem_type, flags, stress).len();
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
