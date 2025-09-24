use crate::{
    declension::{
        AdjectiveDeclension, AnyStemType, DeclensionFlags, NounDeclension, PronounDeclension,
    },
    stress::{AnyDualStress, DUAL_STRESS_MAX_LEN},
    util::UnsafeBuf,
};

// Longest form: °*①②③, ё (16 bytes, 8 chars)
pub const DECLENSION_FLAGS_MAX_LEN: usize = 16;

impl DeclensionFlags {
    #[inline]
    pub(crate) const fn fmt_leading_to(self, dst: &mut [u8; 3]) -> &mut str {
        let mut dst = UnsafeBuf::new(dst);

        if self.has_circle() {
            dst.push('°');
        }
        if self.has_star() {
            dst.push('*');
        }
        dst.finish()
    }
    #[inline]
    pub(crate) const fn fmt_trailing_to(self, dst: &mut [u8; 13]) -> &mut str {
        let mut dst = UnsafeBuf::new(dst);

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
        dst.finish()
    }
    /// Formats these declension flags as UTF-8 into the provided byte buffer, and then returns
    /// a subslice of the buffer that contains the encoded string.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::declension::DeclensionFlags;
    ///
    /// let x = DeclensionFlags::STAR | DeclensionFlags::CIRCLED_ONE;
    /// assert_eq!(x.fmt_to(&mut [0; _]), "*①");
    ///
    /// let x = DeclensionFlags::CIRCLE | DeclensionFlags::ALTERNATING_YO;
    /// assert_eq!(x.fmt_to(&mut [0; _]), "°, ё");
    /// ```
    pub const fn fmt_to(self, dst: &mut [u8; DECLENSION_FLAGS_MAX_LEN]) -> &mut str {
        let mut dst = UnsafeBuf::new(dst);
        dst.push_fmt2(self, Self::fmt_leading_to);
        dst.push_fmt2(self, Self::fmt_trailing_to);
        dst.finish()
    }
}

/// The maximum byte length of a formatted [`DeclensionFlags`].
///
/// Longest form: 6°*f″/f″①②③, ё (26 bytes, 14 chars)
pub const DECLENSION_MAX_LEN: usize = 1 + DECLENSION_FLAGS_MAX_LEN + DUAL_STRESS_MAX_LEN;

const fn fmt_declension_any(
    dst: &mut [u8; DECLENSION_MAX_LEN],
    stem_type: AnyStemType,
    stress: AnyDualStress,
    flags: DeclensionFlags,
) -> &mut str {
    let mut dst = UnsafeBuf::new(dst);

    dst.push(stem_type.to_ascii_digit() as char);
    dst.push_fmt2(flags, DeclensionFlags::fmt_leading_to);
    dst.push_fmt2(stress, AnyDualStress::fmt_to);
    dst.push_fmt2(flags, DeclensionFlags::fmt_trailing_to);

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
        assert_fmt(Type6, S::B_Cp, DF::CIRCLE | DF::STAR | DF::CIRCLED_TWO, "6°*b/c′②");
        assert_fmt(Type1, S::A_Cpp, DF::STAR | DF::CIRCLED_ONE, "1*a/c″①");
        assert_fmt(Type6, S::B_Cpp, DF::all(), "6°*b/c″①②③, ё");
    }
}
