use crate::{
    declension::{
        AdjectiveDeclension, AnyStemType, Declension, DeclensionFlags, DeclensionKind,
        NounDeclension, PronounDeclension,
    },
    stress::{AnyDualStress, ParseStressError},
    util::{PartialFromStr, UnsafeParser, utf8_bytes},
};
use thiserror::Error;

#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum ParseDeclensionError {
    #[error("invalid character in place of stem type")]
    InvalidStemType,
    #[error("error parsing stress: {0}")]
    InvalidStress(ParseStressError),
    #[error("invalid combination or order of flags")]
    InvalidFlags,
    #[error("stem type not compatible with specified type")]
    IncompatibleStemType,
    #[error("stress not compatible with specified type")]
    IncompatibleStress,
    #[error("flags not compatible with specified type")]
    IncompatibleFlags,
    #[error("invalid format")]
    Invalid,
}

impl DeclensionFlags {
    #[inline]
    pub(crate) const fn partial_from_str_leading(flags: &mut Self, parser: &mut UnsafeParser) {
        if parser.skip('°') {
            *flags = flags.union(Self::CIRCLE);
        }
        if parser.skip('*') {
            *flags = flags.union(Self::STAR);
        }
    }
    #[inline]
    pub(crate) const fn partial_from_str_trailing(
        flags: &mut Self,
        parser: &mut UnsafeParser,
    ) -> Result<(), ParseDeclensionError> {
        const CIRCLED_ONE_BYTES: [u8; 3] = utf8_bytes!('①');
        const CIRCLED_TWO_BYTES: [u8; 3] = utf8_bytes!('②');
        const CIRCLED_THREE_BYTES: [u8; 3] = utf8_bytes!('③');

        let mut last_digit = 0u8;
        if matches!(parser.peek_one(), Some(&(226 | b'('))) {
            loop {
                let next_digit = match parser.peek::<3>() {
                    Some(&CIRCLED_ONE_BYTES | b"(1)") => 1,
                    Some(&CIRCLED_TWO_BYTES | b"(2)") => 2,
                    Some(&CIRCLED_THREE_BYTES | b"(3)") => 3,
                    _ => break,
                };
                if next_digit <= last_digit {
                    return Err(Error::InvalidFlags);
                }
                last_digit = next_digit;
                *flags = flags.union(DeclensionFlags::circled_digit(next_digit).unwrap());
                parser.forward(3);
            }
        }

        if parser.skip_str(", ё") {
            *flags = flags.union(DeclensionFlags::ALTERNATING_YO);
        }

        Ok(())
    }
}
impl const PartialFromStr for DeclensionFlags {
    fn partial_from_str(parser: &mut UnsafeParser) -> Result<Self, Self::Err> {
        let mut flags = Self::empty();
        Self::partial_from_str_leading(&mut flags, parser);
        Self::partial_from_str_trailing(&mut flags, parser)?;
        Ok(flags)
    }
}

struct AnyDeclension {
    stem_type: AnyStemType,
    flags: DeclensionFlags,
    stress: AnyDualStress,
}

type Error = ParseDeclensionError;

impl AnyDeclension {
    pub const fn into_noun(self) -> Result<NounDeclension, ParseDeclensionError> {
        Ok(NounDeclension {
            stem_type: self.stem_type.into(),
            stress: self.stress.try_into().ok().ok_or(Error::IncompatibleStress)?,
            flags: self.flags,
        })
    }
    pub const fn into_pronoun(self) -> Result<PronounDeclension, ParseDeclensionError> {
        Ok(PronounDeclension {
            stem_type: self.stem_type.try_into().ok().ok_or(Error::IncompatibleStemType)?,
            stress: self.stress.try_into().ok().ok_or(Error::IncompatibleStress)?,
            flags: self.flags,
        })
    }
    pub const fn into_adjective(self) -> Result<AdjectiveDeclension, ParseDeclensionError> {
        Ok(AdjectiveDeclension {
            stem_type: self.stem_type.try_into().ok().ok_or(Error::IncompatibleStemType)?,
            stress: self.stress.try_into().ok().ok_or(Error::IncompatibleStress)?,
            flags: self.flags,
        })
    }
}

const fn parse_declension_any(parser: &mut UnsafeParser) -> Result<AnyDeclension, Error> {
    let stem_type = match parser.read_one() {
        Some(ch @ b'1'..=b'8') => AnyStemType::from_ascii_digit(*ch).unwrap(),
        _ => return Err(Error::InvalidStemType),
    };

    let mut flags = DeclensionFlags::empty();

    DeclensionFlags::partial_from_str_leading(&mut flags, parser);

    let stress = AnyDualStress::partial_from_str(parser).map_err(Error::InvalidStress)?;

    DeclensionFlags::partial_from_str_trailing(&mut flags, parser)?;

    Ok(AnyDeclension { stem_type, flags, stress })
}

impl const PartialFromStr for NounDeclension {
    fn partial_from_str(parser: &mut UnsafeParser) -> Result<Self, Self::Err> {
        parse_declension_any(parser)?.into_noun()
    }
}
impl const PartialFromStr for PronounDeclension {
    fn partial_from_str(parser: &mut UnsafeParser) -> Result<Self, Self::Err> {
        parse_declension_any(parser)?.into_pronoun()
    }
}
impl const PartialFromStr for AdjectiveDeclension {
    fn partial_from_str(parser: &mut UnsafeParser) -> Result<Self, Self::Err> {
        parse_declension_any(parser)?.into_adjective()
    }
}
impl const PartialFromStr for Declension {
    fn partial_from_str(parser: &mut UnsafeParser) -> Result<Self, Self::Err> {
        let (kind, len) = match parser.peek::<5>() {
            Some(&[0xD0, 0xBC, 0xD1, 0x81, b' ']) => (DeclensionKind::Pronoun, 5), // "мс "
            Some(&[0xD0, 0xBF, b' ', _, _]) => (DeclensionKind::Adjective, 3),     // "п "
            _ => (DeclensionKind::Noun, 0u8),
        };
        parser.forward(len as usize);

        let decl = parse_declension_any(parser)?;

        Ok(match kind {
            DeclensionKind::Noun => Declension::Noun(decl.into_noun()?),
            DeclensionKind::Pronoun => Declension::Pronoun(decl.into_pronoun()?),
            DeclensionKind::Adjective => Declension::Adjective(decl.into_adjective()?),
        })
    }
}

impl std::str::FromStr for DeclensionFlags {
    type Err = ParseDeclensionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_or_err(s, Self::Err::Invalid)
    }
}
impl std::str::FromStr for NounDeclension {
    type Err = ParseDeclensionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_or_err(s, Self::Err::Invalid)
    }
}
impl std::str::FromStr for PronounDeclension {
    type Err = ParseDeclensionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_or_err(s, Self::Err::Invalid)
    }
}
impl std::str::FromStr for AdjectiveDeclension {
    type Err = ParseDeclensionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_or_err(s, Self::Err::Invalid)
    }
}
impl std::str::FromStr for Declension {
    type Err = ParseDeclensionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_or_err(s, Self::Err::Invalid)
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
    fn parse_flags() {
        let assert_ok = |s: &str, flags| assert_eq!(s.parse(), Ok(flags));
        let assert_err = |s: &str, err| assert_eq!(s.parse::<DeclensionFlags>(), Err(err));

        assert_ok("", DF::empty());
        assert_ok("°", DF::CIRCLE);
        assert_ok("*", DF::STAR);
        assert_ok("(1)", DF::CIRCLED_ONE);
        assert_ok("(1)②", DF::CIRCLED_ONE | DF::CIRCLED_TWO);
        assert_ok("①(2)③", DF::CIRCLED_ONE | DF::CIRCLED_TWO | DF::CIRCLED_THREE);
        assert_ok("°②(3)", DF::CIRCLE | DF::CIRCLED_TWO | DF::CIRCLED_THREE);
        assert_ok(", ё", DF::ALTERNATING_YO);
        assert_ok("°, ё", DF::CIRCLE | DF::ALTERNATING_YO);
        assert_ok("°*①②③, ё", DF::all());
        assert_ok("°*(1)②(3), ё", DF::all());

        assert_err("(1)(1)", Error::InvalidFlags);
        assert_err("②②", Error::InvalidFlags);
        assert_err("(2)(1)", Error::InvalidFlags);
        assert_err("(3)(2)", Error::InvalidFlags);
        assert_err("(1)(2)(1)", Error::InvalidFlags);
        assert_err("①②(1)", Error::InvalidFlags);
    }

    #[test]
    fn parse_noun() {
        use {NounStemType::*, NounStress::*};

        let assert_ok = |s: &str, stem_type, stress, flags| {
            assert_eq!(s.parse(), Ok(NounDeclension { stem_type, stress, flags }))
        };
        let assert_err = |s: &str, err| assert_eq!(s.parse::<NounDeclension>(), Err(err));

        assert_ok("1b", Type1, B, DF::empty());
        assert_ok("2a", Type2, A, DF::empty());
        assert_ok("3°d′", Type3, Dp, DF::CIRCLE);
        assert_ok("4*f", Type4, F, DF::STAR);
        assert_ok("5f'(1)", Type5, Fp, DF::CIRCLED_ONE);
        assert_ok("6e②(3)", Type6, E, DF::CIRCLED_TWO | DF::CIRCLED_THREE);
        assert_ok("7f\"(2), ё", Type7, Fpp, DF::CIRCLED_TWO | DF::ALTERNATING_YO);
        assert_ok("8°b′③", Type8, Bp, DF::CIRCLE | DF::CIRCLED_THREE);
        assert_ok("8°*f″①(2)③, ё", Type8, Fpp, DF::all());

        assert_err("", Error::InvalidStemType);
        assert_err("0", Error::InvalidStemType);
        assert_err("9", Error::InvalidStemType);
        assert_err("z", Error::InvalidStemType);
        assert_err("4", Error::InvalidStress(ParseStressError::InvalidLetter));
        assert_err("4z", Error::InvalidStress(ParseStressError::InvalidLetter));
        assert_err("42", Error::InvalidStress(ParseStressError::InvalidLetter));
        assert_err("4b″", Error::InvalidStress(ParseStressError::InvalidPrime));
        assert_err("4a′", Error::IncompatibleStress);
        assert_err("4a/a", Error::IncompatibleStress);
    }

    #[test]
    fn parse_pronoun() {
        use {PronounStemType::*, PronounStress::*};

        let assert_ok = |s: &str, stem_type, stress, flags| {
            assert_eq!(s.parse(), Ok(PronounDeclension { stem_type, stress, flags }))
        };
        let assert_err = |s: &str, err| assert_eq!(s.parse::<PronounDeclension>(), Err(err));

        assert_ok("1a", Type1, A, DF::empty());
        assert_ok("2f(1)", Type2, F, DF::CIRCLED_ONE);
        assert_ok("4°*b", Type4, B, DF::CIRCLE | DF::STAR);
        assert_ok("6°*f①②(3), ё", Type6, F, DF::all());

        assert_err("2c", Error::IncompatibleStress);
        assert_err("2a/a", Error::IncompatibleStress);
    }

    #[test]
    fn parse_adjective() {
        use {AdjectiveStemType::*, AdjectiveStress as S};

        let assert_ok = |s: &str, stem_type, stress, flags| {
            assert_eq!(s.parse(), Ok(AdjectiveDeclension { stem_type, stress, flags }))
        };
        let assert_err = |s: &str, err| assert_eq!(s.parse::<AdjectiveDeclension>(), Err(err));

        assert_ok("1b/b", Type1, S::B_B, DF::empty());
        assert_ok("2°a", Type2, S::A_A, DF::CIRCLE);
        assert_ok("3*b/a", Type3, S::B_A, DF::STAR);
        assert_ok("4b(1)②", Type4, S::B_B, DF::CIRCLED_ONE | DF::CIRCLED_TWO);
        assert_ok("5b/c(1), ё", Type5, S::B_C, DF::CIRCLED_ONE | DF::ALTERNATING_YO);
        assert_ok("6a/c′②", Type6, S::A_Cp, DF::CIRCLED_TWO);
        assert_ok("7°*b/c''(1)(2)③, ё", Type7, S::B_Cpp, DF::all());

        assert_err("2c", Error::IncompatibleStress);
        assert_err("2a/f", Error::IncompatibleStress);
    }

    #[test]
    fn parse_declension() {
        let decl = Declension::Noun(NounDeclension {
            stem_type: NounStemType::Type3,
            stress: NounStress::Bp,
            flags: DeclensionFlags::CIRCLE | DeclensionFlags::ALTERNATING_YO,
        });
        assert_eq!("3°b′, ё".parse(), Ok(decl));

        let decl = Declension::Pronoun(PronounDeclension {
            stem_type: PronounStemType::Type6,
            stress: PronounStress::F,
            flags: DeclensionFlags::STAR | DeclensionFlags::CIRCLED_ONE,
        });
        assert_eq!("мс 6*f(1)".parse(), Ok(decl));

        let decl = Declension::Adjective(AdjectiveDeclension {
            stem_type: AdjectiveStemType::Type4,
            stress: AdjectiveStress::B_Ap,
            flags: DeclensionFlags::STAR | DeclensionFlags::CIRCLED_TWO,
        });
        assert_eq!("п 4*b/a′(2)".parse(), Ok(decl));
    }
}
