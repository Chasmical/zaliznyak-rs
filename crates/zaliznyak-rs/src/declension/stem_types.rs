use crate::{util::enum_conversion, word::Utf8Letter};
use thiserror::Error;

macro_rules! impl_stem_type {
    (
        $(#[$outer:meta])*
        $vis:vis enum $T:ident {
            $( $(#[$inner:meta])* $variant:ident => $value:expr ),+ $(,)?
        }
    ) => (
        $(#[$outer])*
        #[derive(Debug, Copy, Eq, Hash)]
        #[derive_const(Clone, PartialEq)]
        $vis enum $T {
            $( $(#[$inner])* $variant,)+
        }

        impl $T {
            pub const fn from_digit(digit: u8) -> Option<Self> {
                Some(match digit { $($value => <$T>::$variant,)+ _ => return None })
            }
            pub const fn from_ascii_digit(ascii_digit: u8) -> Option<Self> {
                Self::from_digit(ascii_digit - b'0')
            }
            pub const fn to_digit(self) -> u8 {
                match self { $(<$T>::$variant => $value,)+ }
            }
            pub const fn to_ascii_digit(self) -> u8 {
                b'0' + self.to_digit()
            }
        }
        impl std::fmt::Display for $T {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let ascii_digit = self.to_ascii_digit();
                let slice = std::slice::from_ref(&ascii_digit);
                unsafe { str::from_utf8_unchecked(slice) }.fmt(f)
            }
        }
        impl std::str::FromStr for $T {
            type Err = ParseStemTypeError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if let [ch] = s.as_bytes() {
                    Ok(match ch - b'0' {
                        $($value => <$T>::$variant,)+
                        0..=9 => return Err(Self::Err::IncompatibleDigit),
                        _ => return Err(Self::Err::Invalid),
                    })
                } else {
                    Err(Self::Err::Invalid)
                }
            }
        }
    );
}

/// Error type for parsing various stem types.
#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum ParseStemTypeError {
    /// The parsed value is not compatible with specified stem type.
    #[error("digit not compatible with specified type")]
    IncompatibleDigit,
    /// Invalid format.
    #[error("invalid format")]
    Invalid,
}

/// Error type for conversion to [`AnyStemType`].
#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
#[error("words can only have stem types 1 through 8")]
pub struct AnyStemTypeError;
/// Error type for conversion to [`NounStemType`].
#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
#[error("nouns can only have stem types 1 through 8")]
pub struct NounStemTypeError;
/// Error type for conversion to [`PronounStemType`].
#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
#[error("pronouns can only have stem types 1, 2, 4 and 6")]
pub struct PronounStemTypeError;
/// Error type for conversion to [`AdjectiveStemType`].
#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
#[error("adjectives can only have stem types 1 through 6")]
pub struct AdjectiveStemTypeError;

impl_stem_type! {
    /// Any word's stem type. Can be converted to and from any other stem type.
    /// [See the dictionary for more details](https://gramdict.ru/declension/symbols#meaning1).
    pub enum AnyStemType {
        Type1 => 1, Type2 => 2, Type3 => 3, Type4 => 4,
        Type5 => 5, Type6 => 6, Type7 => 7, Type8 => 8,
    }
}
impl_stem_type! {
    /// A non stem type.
    /// [See the dictionary for more details](https://gramdict.ru/declension/symbols#meaning1).
    pub enum NounStemType {
        Type1 => 1, Type2 => 2, Type3 => 3, Type4 => 4,
        Type5 => 5, Type6 => 6, Type7 => 7, Type8 => 8,
    }
}
impl_stem_type! {
    /// A pronoun stem type.
    /// [See the dictionary for more details](https://gramdict.ru/declension/symbols#meaning1).
    pub enum PronounStemType {
        Type1 => 1, Type2 => 2, Type4 => 4, Type6 => 6,
    }
}
impl_stem_type! {
    /// An adjective stem type.
    /// [See the dictionary for more details](https://gramdict.ru/declension/symbols#meaning1).
    pub enum AdjectiveStemType {
        Type1 => 1, Type2 => 2, Type3 => 3, Type4 => 4,
        Type5 => 5, Type6 => 6,
    }
}

enum_conversion! {
    AnyStemType => NounStemType { Type1, Type2, Type3, Type4, Type5, Type6, Type7, Type8 }
}
enum_conversion! {
    AnyStemType => PronounStemType { Type1, Type2, Type4, Type6 } else { PronounStemTypeError }
}
enum_conversion! {
    AnyStemType => AdjectiveStemType { Type1, Type2, Type3, Type4, Type5, Type6 }
    else { AdjectiveStemTypeError }
}

const fn is_trim_letter(letter: Utf8Letter) -> bool {
    use Utf8Letter::*;
    matches!(letter, А | Е | И | Й | О | У | Ы | Ь | Э | Ю | Я | Ё)
}
const fn identify_stem_type(
    stem: Utf8Letter,
    after: Option<Utf8Letter>,
) -> Option<AnyStemType> {
    use Utf8Letter::*;

    Some(match stem {
        Г | К | Х => AnyStemType::Type3,
        Ж | Ч | Ш | Щ => AnyStemType::Type4,
        Ц => AnyStemType::Type5,
        А | Е | Й | О | У | Ы | Ь | Э | Ю | Я | Ё => AnyStemType::Type6,
        И => AnyStemType::Type7,
        Б | В | Д | З | Л | М | Н | П | Р | С | Т | Ф => {
            let hard = matches!(after, None | Some(А | О | У | Ы | Э));
            if hard { AnyStemType::Type1 } else { AnyStemType::Type2 }
        },
        Ъ => return None,
    })
}

impl NounStemType {
    /// Identifies a noun's stem and stem type from its nominative form.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::declension::NounStemType;
    ///
    /// assert_eq!(NounStemType::identify("акулы"), Some(("акул", NounStemType::Type1)));
    /// assert_eq!(NounStemType::identify("точка"), Some(("точк", NounStemType::Type3)));
    /// assert_eq!(NounStemType::identify("блюдца"), Some(("блюдц", NounStemType::Type5)));
    /// assert_eq!(NounStemType::identify("литий"), Some(("лити", NounStemType::Type7)));
    ///
    /// assert_eq!(NounStemType::identify("циркъ"), None);
    /// assert_eq!(NounStemType::identify("wxyz"), None);
    /// assert_eq!(NounStemType::identify("ы"), None);
    /// assert_eq!(NounStemType::identify(""), None);
    /// ```
    pub const fn identify(word: &str) -> Option<(&str, NounStemType)> {
        // Read the word's last char
        let (word_without_last, last) = Utf8Letter::split_last(word)?;

        let (stem, stem_char, after) = {
            // If the last char is trimmable, exclude it from stem
            if is_trim_letter(last) {
                // Read the actual last stem char
                let stem_char = Utf8Letter::split_last(word_without_last)?.1;
                (word_without_last, stem_char, Some(last))
            } else {
                (word, last, None)
            }
        };

        // Identify the stem type from letters
        let stem_type = identify_stem_type(stem_char, after)?;
        Some((stem, stem_type.into()))
    }
}

impl PronounStemType {
    /// Identifies a pronoun's stem and stem type from its nominative form.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::declension::PronounStemType;
    ///
    /// assert_eq!(PronounStemType::identify("один"), Some(("один", PronounStemType::Type1)));
    /// assert_eq!(PronounStemType::identify("наши"), Some(("наш", PronounStemType::Type4)));
    /// assert_eq!(PronounStemType::identify("своё"), Some(("сво", PronounStemType::Type6)));
    ///
    /// assert_eq!(PronounStemType::identify("точки"), None); // 3 - not compatible with pronouns
    /// assert_eq!(PronounStemType::identify("солнце"), None); // 5 - not compatible with pronouns
    /// assert_eq!(PronounStemType::identify("чий"), None); // 7 - not compatible with pronouns
    ///
    /// assert_eq!(PronounStemType::identify("wxyz"), None);
    /// assert_eq!(PronounStemType::identify("ы"), None);
    /// assert_eq!(PronounStemType::identify(""), None);
    /// ```
    pub const fn identify(word: &str) -> Option<(&str, PronounStemType)> {
        let (stem, stem_type) = NounStemType::identify(word)?;
        Some((stem, AnyStemType::from(stem_type).try_into().ok()?))
    }
}

impl AdjectiveStemType {
    /// Identifies an adjective's stem and stem type from its nominative form, also returning a
    /// `bool` indicating whether the adjective is reflexive or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::declension::AdjectiveStemType;
    ///
    /// assert_eq!(
    ///     AdjectiveStemType::identify("красный"),
    ///     Some(("красн", AdjectiveStemType::Type1, false)),
    /// );
    /// assert_eq!(
    ///     AdjectiveStemType::identify("мягкая"),
    ///     Some(("мягк", AdjectiveStemType::Type3, false)),
    /// );
    /// assert_eq!(
    ///     AdjectiveStemType::identify("светящееся"),
    ///     Some(("светящ", AdjectiveStemType::Type4, true)),
    /// );
    /// // TODO: should -ся be allowed for non-type-4 words?
    /// assert_eq!(
    ///     AdjectiveStemType::identify("красныйся"),
    ///     Some(("красн", AdjectiveStemType::Type1, true)),
    /// );
    ///
    /// assert_eq!(AdjectiveStemType::identify("ниий"), None); // 7 - not compatible with adjectives
    ///
    /// assert_eq!(AdjectiveStemType::identify("wxyz"), None);
    /// assert_eq!(AdjectiveStemType::identify("ы"), None);
    /// assert_eq!(AdjectiveStemType::identify(""), None);
    /// ```
    pub const fn identify(word: &str) -> Option<(&str, AdjectiveStemType, bool)> {
        let (word, is_reflexive) = {
            // Remove 'ся' suffix from reflexive adjectives

            // FIXME(const-hack): Replace with `.strip_suffix().map_or((word, false), |x| (x, true))`.
            if let Some((stripped, last)) = word.as_bytes().split_last_chunk::<4>()
                && last as &[u8] == "ся".as_bytes()
            {
                (unsafe { str::from_utf8_unchecked(stripped) }, true)
            } else {
                (word, false)
            }
        };

        // Read the word's two ending chars
        let (word, _ending_last_char) = Utf8Letter::split_last(word)?;
        let (word, ending_first_char) = Utf8Letter::split_last(word)?;

        // Read the stem's last char
        let (_, stem_char) = Utf8Letter::split_last(word)?;

        // Identify the stem type from letters
        let stem_type = identify_stem_type(stem_char, Some(ending_first_char))?;
        Some((word, stem_type.try_into().ok()?, is_reflexive))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identify_noun() {
        use NounStemType as ST;

        assert_eq!(ST::identify("ведро"), Some(("ведр", ST::Type1)));
        assert_eq!(ST::identify("тополь"), Some(("топол", ST::Type2)));
        assert_eq!(ST::identify("сапог"), Some(("сапог", ST::Type3)));
        assert_eq!(ST::identify("дача"), Some(("дач", ST::Type4)));
        assert_eq!(ST::identify("яйцо"), Some(("яйц", ST::Type5)));
        assert_eq!(ST::identify("бельё"), Some(("бель", ST::Type6)));
        assert_eq!(ST::identify("литий"), Some(("лити", ST::Type7)));

        assert_eq!(ST::identify("воръ"), None);
        assert_eq!(ST::identify("noun"), None);
        assert_eq!(ST::identify("nounя"), None);
    }

    #[test]
    fn identify_pro() {
        use PronounStemType as ST;

        assert_eq!(ST::identify("отцов"), Some(("отцов", ST::Type1)));
        assert_eq!(ST::identify("господень"), Some(("господен", ST::Type2)));
        assert_eq!(ST::identify("наш"), Some(("наш", ST::Type4)));
        assert_eq!(ST::identify("твой"), Some(("тво", ST::Type6)));

        assert_eq!(ST::identify("сёмга"), None); // stem type 3 - incompatible
        assert_eq!(ST::identify("блюдце"), None); // stem type 5 - incompatible
        assert_eq!(ST::identify("усилие"), None); // stem type 7 - incompatible
    }

    #[test]
    fn identify_adj() {
        use AdjectiveStemType as ST;

        assert_eq!(ST::identify("живой"), Some(("жив", ST::Type1, false)));
        assert_eq!(ST::identify("осенний"), Some(("осенн", ST::Type2, false)));
        assert_eq!(ST::identify("плавкий"), Some(("плавк", ST::Type3, false)));
        assert_eq!(ST::identify("светящийся"), Some(("светящ", ST::Type4, true)));
        assert_eq!(ST::identify("куцый"), Some(("куц", ST::Type5, false)));
        assert_eq!(ST::identify("голошеий"), Some(("голоше", ST::Type6, false)));

        assert_eq!(ST::identify("сиой"), None); // not a real adjective, stem type 7 - incompatible

        assert_eq!(ST::identify("серъое"), None);
        assert_eq!(ST::identify("adjective"), None);
        assert_eq!(ST::identify("adjectiveое"), None);
    }
}
