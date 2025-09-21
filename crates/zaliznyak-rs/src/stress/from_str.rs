use crate::{
    stress::{
        AdjectiveFullStress, AdjectiveShortStress, AdjectiveStress, AnyDualStress, AnyStress,
        NounStress, PronounStress, VerbPastStress, VerbPresentStress, VerbStress,
    },
    util::{PartialFromStr, UnsafeParser},
};
use thiserror::Error;

/// Error type for parsing various stress types.
#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum ParseStressError {
    /// The first character is not a valid latin letter.
    #[error("invalid character in place of letter")]
    InvalidLetter,
    /// Attempted to construct a double-primed stress othen than c″ and f″.
    #[error("invalid combination of letter and primes")]
    InvalidPrime,
    /// The parsed value is not compatible with specified stress type.
    #[error("stress not compatible with specified type")]
    Incompatible,
    /// Invalid format.
    #[error("invalid format")]
    Invalid,
}

impl const PartialFromStr for AnyStress {
    fn partial_from_str(parser: &mut UnsafeParser) -> Result<Self, Self::Err> {
        // Parse the latin letter
        let letter = match parser.read_one() {
            Some(b'a') => Self::A,
            Some(b'b') => Self::B,
            Some(b'c') => Self::C,
            Some(b'd') => Self::D,
            Some(b'e') => Self::E,
            Some(b'f') => Self::F,
            _ => return Err(ParseStressError::InvalidLetter),
        };

        // Then parse prime indicators
        let (primes, primes_len) = match parser.remaining() {
            [0xE2, 0x80, 0xB2, ..] => (1, 3), // ′ (UTF-8 single prime)
            [0xE2, 0x80, 0xB3, ..] => (2, 3), // ″ (UTF-8 double prime)
            [b'\'', b'\'', ..] => (2, 2),     // '' (double apostrophe)
            [b'\'', ..] => (1, 1),            // ' (apostrophe)
            [b'"', ..] => (2, 1),             // " (quotation)
            _ => (0u8, 0u8),                  // no primes
        };
        parser.forward(primes_len as usize);

        // Try to add the parsed amount of primes to the letter, and return
        Ok(match primes {
            0 => letter,
            1 => letter.add_single_prime().ok_or(ParseStressError::InvalidPrime)?,
            2 => letter.add_double_prime().ok_or(ParseStressError::InvalidPrime)?,
            _ => unreachable!(),
        })
    }
}
impl const PartialFromStr for AnyDualStress {
    fn partial_from_str(parser: &mut UnsafeParser) -> Result<Self, Self::Err> {
        // Parse the main stress
        let main = AnyStress::partial_from_str(parser)?;
        let mut alt = None;

        // If followed by '/', parse the alt stress
        if parser.skip('/') {
            alt = Some(AnyStress::partial_from_str(parser)?);
        }

        Ok(Self::new(main, alt))
    }
}

impl std::str::FromStr for AnyStress {
    type Err = ParseStressError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_or_err(s, ParseStressError::Invalid)
    }
}
impl std::str::FromStr for AnyDualStress {
    type Err = ParseStressError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_or_err(s, ParseStressError::Invalid)
    }
}

macro_rules! derive_simple_from_str_impls {
    ($(
        $any:ty { $($t:ty),+ $(,)? }
    )+) => ($($(
        impl std::str::FromStr for $t {
            type Err = ParseStressError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                <$any>::from_str(s)?.try_into().or(Err(Self::Err::Incompatible))
            }
        }
    )+)+);
}
derive_simple_from_str_impls! {
    AnyStress {
        NounStress, PronounStress, AdjectiveFullStress, AdjectiveShortStress, VerbPresentStress, VerbPastStress,
    }
    AnyDualStress {
        AdjectiveStress, VerbStress,
    }
}

#[cfg(test)]
mod tests {
    use super::{ParseStressError as Error, *};

    #[test]
    fn parse_any() {
        assert_eq!("a".parse::<AnyStress>(), Ok(AnyStress::A));
        assert_eq!("f".parse::<AnyStress>(), Ok(AnyStress::F));
        assert_eq!("e'".parse::<AnyStress>(), Ok(AnyStress::Ep));
        assert_eq!("c\"".parse::<AnyStress>(), Ok(AnyStress::Cpp));
        assert_eq!("a′".parse::<AnyStress>(), Ok(AnyStress::Ap));
        assert_eq!("c''".parse::<AnyStress>(), Ok(AnyStress::Cpp));
        assert_eq!("f″".parse::<AnyStress>(), Ok(AnyStress::Fpp));

        assert_eq!("".parse::<AnyStress>(), Err(Error::InvalidLetter));
        assert_eq!("/".parse::<AnyStress>(), Err(Error::InvalidLetter));
        assert_eq!("a/".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("/b".parse::<AnyStress>(), Err(Error::InvalidLetter));
        assert_eq!("a/b".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("z".parse::<AnyStress>(), Err(Error::InvalidLetter));
        assert_eq!("A".parse::<AnyStress>(), Err(Error::InvalidLetter));
        assert_eq!("ab".parse::<AnyStress>(), Err(Error::Invalid));
        assert_eq!("$a".parse::<AnyStress>(), Err(Error::InvalidLetter));
        assert_eq!("a$".parse::<AnyStress>(), Err(Error::Invalid));
    }
    #[test]
    fn parse_dual() {
        use AnyStress::*;

        assert_eq!("a".parse::<AnyDualStress>(), Ok(A.into()));
        assert_eq!("f".parse::<AnyDualStress>(), Ok(F.into()));
        assert_eq!("e'".parse::<AnyDualStress>(), Ok(Ep.into()));
        assert_eq!("c\"".parse::<AnyDualStress>(), Ok(Cpp.into()));
        assert_eq!("a/b".parse::<AnyDualStress>(), Ok((A, B).into()));
        assert_eq!("d'/b′".parse::<AnyDualStress>(), Ok((Dp, Bp).into()));
        assert_eq!("e′/c\"".parse::<AnyDualStress>(), Ok((Ep, Cpp).into()));
        assert_eq!("f″/e'".parse::<AnyDualStress>(), Ok((Fpp, Ep).into()));
        assert_eq!("e′/c''".parse::<AnyDualStress>(), Ok((Ep, Cpp).into()));

        assert_eq!("".parse::<AnyDualStress>(), Err(Error::InvalidLetter));
        assert_eq!("/".parse::<AnyDualStress>(), Err(Error::InvalidLetter));
        assert_eq!("a/".parse::<AnyDualStress>(), Err(Error::InvalidLetter));
        assert_eq!("/b".parse::<AnyDualStress>(), Err(Error::InvalidLetter));
        assert_eq!("z".parse::<AnyDualStress>(), Err(Error::InvalidLetter));
        assert_eq!("a/z".parse::<AnyDualStress>(), Err(Error::InvalidLetter));
        assert_eq!("A".parse::<AnyDualStress>(), Err(Error::InvalidLetter));
        assert_eq!("ab".parse::<AnyDualStress>(), Err(Error::Invalid));
        assert_eq!("$a/b".parse::<AnyDualStress>(), Err(Error::InvalidLetter));
        assert_eq!("a/b$".parse::<AnyDualStress>(), Err(Error::Invalid));
    }
}
