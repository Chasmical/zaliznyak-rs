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
