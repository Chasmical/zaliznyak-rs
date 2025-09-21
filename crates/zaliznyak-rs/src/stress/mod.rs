//! Stress schema types, according to Zaliznyak's classification.
//!
//! # Stress schemas
//!
//! There are a total of 13 stress schemas: 6 primary letters (a, b, c, d, e, f), 5 with single
//! prime (a′, b′, c′, d′, f′), and 2 with double prime (c″ and f″). Stress schemas with primes
//! represent some deviations from the primary 6. In stress schema a, the stress always falls on
//! the stem, and in stress schema b --- on the ending. The rest vary by part of speech.
//!
//! Nouns and pronouns have singular stress schemas (a, b′, f″), and adjectives and verbs have
//! dual stress schemas (a/b, b/c′, c″/b), that can sometimes be abbreviated to just one letter
//! (e.g. adjectives: a/a --- a, b/b′ --- b′; and verbs: a/a --- a, c/a --- c). Each word form's
//! stress schema also defines methods for determining if the inflected word's stem or ending
//! should be stressed or not.
//!
//! For more information about the stress schemas, stress placement and schema abbreviation rules,
//! see the corresponding sections in Zaliznyak's dictionary:
//! [noun stresses](https://gramdict.ru/declension/symbols#latin1),
//! [pronoun stresses](https://gramdict.ru/declension/symbols#latin3),
//! [adjective stresses](https://gramdict.ru/declension/symbols#latin2),
//! [verb stresses](https://gramdict.ru/conjugation#latin-letter).
//!
//! # `AnyStress` and `AnyDualStress`
//!
//! To allow comparing different types of stresses, this module also provides [`AnyStress`] and
//! [`AnyDualStress`] types as common interfaces for singular and dual stress schemas accordingly.
//! All stress types can be converted to and from these types.
//!
//! ```
//! use zaliznyak::stress::*;
//!
//! // Same letters are converted to same values
//! let x: AnyStress = NounStress::Bp.into();
//! let y: AnyStress = AdjectiveShortStress::Bp.into();
//! assert_eq!(x, y);
//!
//! // CAUTION: adjective and verb stresses are abbreviated using different rules!
//! // For example, adjective stress 'b' is an abbreviation for 'b/b', while verb
//! // stress 'b' is an abbreviation for 'b/a', resulting in different values.
//! let x: AnyDualStress = AdjectiveStress::B.into();
//! let y: AnyDualStress = VerbStress::B.into();
//! assert!(x != y);
//!
//! // To compare dual stresses "visually", you can instead use their .abbr() methods,
//! // which return `None` in the second field if the dual stress is abbreviatable.
//! let x: AnyDualStress = AdjectiveStress::B.abbr();
//! let y: AnyDualStress = VerbStress::B.abbr();
//! assert_eq!(x, y);
//! ```
//!
//! # Parsing and formatting
//!
//! All stress types also support parsing and formatting:
//!
//! ```
//! use zaliznyak::stress::*;
//!
//! let x: NounStress = "a".parse().unwrap();
//! assert_eq!(x, NounStress::A);
//! assert_eq!(x.to_string(), "a");
//!
//! let x: AnyDualStress = "f\"/f''".parse().unwrap();
//! assert_eq!(x, (AnyStress::Fpp, AnyStress::Fpp).into());
//! assert_eq!(x.to_string(), "f″/f″");
//! ```

mod convert;
mod fmt;
mod from_str;
mod methods;

pub use convert::*;
pub use fmt::*;
pub use from_str::*;

/// Any word's single stress schema. Can be converted to and from any other stress type.
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum AnyStress {
    /// Stress schema a. The stress is always on the stem. Used by all inflectable words.
    A = 1,
    /// Stress schema b. The stress is always on the ending. Used by all inflectable words.
    B,
    /// Stress schema c. Used by nouns, adjectives' short forms and verbs.
    C,
    /// Stress schema d. Used by nouns.
    D,
    /// Stress schema e. Used by nouns.
    E,
    /// Stress schema f. Used by nouns and pronouns.
    F,
    /// Stress schema a′ (a with single prime). Used by adjectives' short forms.
    Ap,
    /// Stress schema b′ (b with single prime). Used by nouns and adjectives' short forms.
    Bp,
    /// Stress schema c′ (c with single prime). Used by adjectives' short forms and verbs.
    Cp,
    /// Stress schema d′ (d with single prime). Used by nouns.
    Dp,
    /// Stress schema e′ (e with single prime). Unused.
    Ep,
    /// Stress schema f′ (f with single prime). Used by nouns.
    Fp,
    /// Stress schema c″ (c with double prime). Used by adjectives' short forms and verbs.
    Cpp,
    /// Stress schema f″ (f with double prime). Used by nouns.
    Fpp,
}

/// A noun stress schema.
/// [See the dictionary for more details](https://gramdict.ru/declension/symbols#latin1).
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum NounStress {
    /// Stress schema a. The stress is always on the stem.
    A,
    /// Stress schema b. The stress is always on the ending.
    B,
    /// Stress schema c. Singular --- on stem, plural --- on ending.
    C,
    /// Stress schema d. Singular --- on ending, plural --- on stem.
    D,
    /// Stress schema e. Singular, and plural nominative --- on stem, plural of other cases --- on ending.
    E,
    /// Stress schema f. Plural nominative --- on stem, all other --- on ending.
    F,
    /// Stress schema b′ (b with single prime). Singular instrumental --- on stem, all other --- on ending.
    Bp,
    /// Stress schema d′ (d with single prime). Singular accusative, and plural --- on stem, singular of other cases --- on ending.
    Dp,
    /// Stress schema f′ (f with single prime). Singular accusative, and plural nominative --- on stem, all other --- on ending.
    Fp,
    /// Stress schema f″ (f with double prime). Singular instrumental, and plural nominative --- on stem, all other --- on ending.
    Fpp,
}

/// A pronoun stress schema.
/// [See the dictionary for more details](https://gramdict.ru/declension/symbols#latin3).
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum PronounStress {
    /// Stress schema a. The stress is always on the stem.
    A,
    /// Stress schema b. The stress is always on the ending.
    B,
    /// Stress schema f. Plural nominative --- on stem, all other --- on ending.
    F,
}

/// An adjective's full form stress schema.
/// [See the dictionary for more details](https://gramdict.ru/declension/symbols#full-adj-stress-scheme).
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum AdjectiveFullStress {
    /// Stress schema a. The stress is always on the stem.
    A,
    /// Stress schema b. The stress is always on the ending.
    B,
}

/// An adjective's short form stress schema.
/// [See the dictionary for more details](https://gramdict.ru/declension/symbols#short-adj-stress-scheme).
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum AdjectiveShortStress {
    /// Stress schema a. The stress is always on the stem.
    A,
    /// Stress schema b. The stress is always on the ending.
    B,
    /// Stress schema c. Feminine --- on ending, all other --- on stem.
    C,
    /// Stress schema a′ (a with single prime). Feminine --- either, all other --- on stem.
    Ap,
    /// Stress schema b′ (b with single prime). Plural --- either, all other --- on ending.
    Bp,
    /// Stress schema c′ (c with single prime). Feminine --- on ending, neuter --- on stem, plural --- either.
    Cp,
    /// Stress schema c″ (c with double prime). Feminine --- on ending, all other --- either.
    Cpp,
}

/// A verb's present tense form stress schema.
/// [See the dictionary for more details](https://gramdict.ru/conjugation#present-stress-types).
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum VerbPresentStress {
    /// Stress schema a. The stress is always on the stem.
    A,
    /// Stress schema b. The stress is always on the ending.
    B,
    /// Stress schema c. First person, and imperative --- on ending, all other --- on stem.
    C,
    /// Stress schema c′ (c with single prime). First person, imperative, and plural --- on ending, all other --- on stem.
    Cp,
}

/// A verb's past tense form stress schema.
/// [See the dictionary for more details](https://gramdict.ru/conjugation#past-stress-types).
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub enum VerbPastStress {
    /// Stress schema a. The stress is always on the stem.
    A,
    /// Stress schema b. The stress is always on the ending.
    B,
    /// Stress schema c. Feminine --- on ending, all other --- on stem.
    C,
    /// Stress schema c′ (c with single prime). Feminine --- on ending, neuter --- either, all other --- on stem.
    Cp,
    /// Stress schema c″ (c with double prime). Past tense reflexive only. Masculine --- on stem, feminine --- on ending, all other --- either.
    Cpp,
}

/// Any word's dual stress schema. Can be converted to and from any other stress type.
///
/// # Examples
///
/// ```
/// use zaliznyak::stress::{AdjectiveShortStress, AnyDualStress, AnyStress, VerbStress};
///
/// // You can explicitly call the constructor:
/// let dual = AnyDualStress::new(AnyStress::A, Some(AnyStress::B));
///
/// // Or convert any other single or dual stress type into it:
/// let dual: AnyDualStress = AdjectiveShortStress::Bp.into();
/// let dual: AnyDualStress = VerbStress::Cp_B.into();
///
/// // But you can also construct `AnyDualStress` through tuples,
/// // when the expression's type can be inferred from context:
/// let dual: AnyDualStress = (AnyStress::A, Some(AnyStress::B)).into();
/// let dual: AnyDualStress = (AnyStress::A, AnyStress::B).into();
/// let dual: AnyDualStress = AnyStress::A.into();
///
/// // Or you can parse the value from a string:
/// let dual: AnyDualStress = "a/b".parse().unwrap();
/// ```
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub struct AnyDualStress {
    /// The main form's stress schema.
    pub main: AnyStress,
    /// The optional alternative form's stress schema.
    pub alt: Option<AnyStress>,
}

/// A complete adjective stress schema, containing [full][AdjectiveFullStress]
/// and [short form][AdjectiveShortStress] stress schemas.
///
/// # Examples
///
/// `AdjectiveStress` provides constants for all possible values for convenience:
///
/// ```
/// use zaliznyak::stress::{AdjectiveFullStress, AdjectiveShortStress, AdjectiveStress};
///
/// let stress = AdjectiveStress::A_Bp; // a/b′
/// assert_eq!(stress.full, AdjectiveFullStress::A);
/// assert_eq!(stress.short, AdjectiveShortStress::Bp);
/// ```
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub struct AdjectiveStress {
    /// The full form's stress schema.
    pub full: AdjectiveFullStress,
    /// The short form's stress schema.
    pub short: AdjectiveShortStress,
}

/// A complete verb stress schema, containing [present][VerbPresentStress]
/// and [past tense form][VerbPastStress] stress schemas.
///
/// # Examples
///
/// `VerbStress` provides constants for all possible values for convenience:
///
/// ```
/// use zaliznyak::stress::{VerbPastStress, VerbPresentStress, VerbStress};
///
/// let stress = VerbStress::B_Cp; // b/c′
/// assert_eq!(stress.present, VerbPresentStress::B);
/// assert_eq!(stress.past, VerbPastStress::Cp);
/// ```
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Clone, PartialEq)]
pub struct VerbStress {
    /// The present tense form's stress schema.
    pub present: VerbPresentStress,
    /// The past tense form's stress schema.
    pub past: VerbPastStress,
}

impl AnyDualStress {
    /// Constructs a new `AnyDualStress` from provided stress schemas.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::{AnyDualStress, AnyStress};
    ///
    /// let dual = AnyDualStress::new(AnyStress::A, Some(AnyStress::Bp));
    /// ```
    #[must_use]
    pub const fn new(main: AnyStress, alt: Option<AnyStress>) -> Self {
        Self { main, alt }
    }
}
impl AdjectiveStress {
    /// Constructs a new `AdjectiveStress` from provided stress schemas.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::{AdjectiveFullStress, AdjectiveShortStress, AdjectiveStress};
    ///
    /// let stress = AdjectiveStress::new(AdjectiveFullStress::A, AdjectiveShortStress::Cp);
    /// ```
    #[must_use]
    pub const fn new(full: AdjectiveFullStress, short: AdjectiveShortStress) -> Self {
        Self { full, short }
    }
}
impl VerbStress {
    /// Constructs a new `VerbStress` from provided stress schemas.
    ///
    /// # Examples
    ///
    /// ```
    /// use zaliznyak::stress::{VerbPastStress, VerbPresentStress, VerbStress};
    ///
    /// let stress = VerbStress::new(VerbPresentStress::C, VerbPastStress::A);
    /// ```
    #[must_use]
    pub const fn new(present: VerbPresentStress, past: VerbPastStress) -> Self {
        Self { present, past }
    }
}

#[allow(non_upper_case_globals)]
impl AdjectiveStress {
    pub const A: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::A);
    pub const A_A: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::A);
    pub const A_B: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::B);
    pub const A_C: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::C);
    pub const A_Ap: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::Ap);
    pub const A_Bp: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::Bp);
    pub const A_Cp: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::Cp);
    pub const A_Cpp: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::Cpp);

    pub const B: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::B);
    pub const B_A: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::A);
    pub const B_B: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::B);
    pub const B_C: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::C);
    pub const B_Ap: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::Ap);
    pub const B_Bp: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::Bp);
    pub const B_Cp: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::Cp);
    pub const B_Cpp: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::Cpp);

    pub const Ap: Self = Self::new(AdjectiveFullStress::A, AdjectiveShortStress::Ap);
    pub const Bp: Self = Self::new(AdjectiveFullStress::B, AdjectiveShortStress::Bp);
}
#[allow(non_upper_case_globals)]
impl VerbStress {
    pub const A: Self = Self::new(VerbPresentStress::A, VerbPastStress::A);
    pub const A_A: Self = Self::new(VerbPresentStress::A, VerbPastStress::A);
    pub const A_B: Self = Self::new(VerbPresentStress::A, VerbPastStress::B);
    pub const A_C: Self = Self::new(VerbPresentStress::A, VerbPastStress::C);
    pub const A_Cp: Self = Self::new(VerbPresentStress::A, VerbPastStress::Cp);
    pub const A_Cpp: Self = Self::new(VerbPresentStress::A, VerbPastStress::Cpp);

    pub const B: Self = Self::new(VerbPresentStress::B, VerbPastStress::A);
    pub const B_A: Self = Self::new(VerbPresentStress::B, VerbPastStress::A);
    pub const B_B: Self = Self::new(VerbPresentStress::B, VerbPastStress::B);
    pub const B_C: Self = Self::new(VerbPresentStress::B, VerbPastStress::C);
    pub const B_Cp: Self = Self::new(VerbPresentStress::B, VerbPastStress::Cp);
    pub const B_Cpp: Self = Self::new(VerbPresentStress::B, VerbPastStress::Cpp);

    pub const C: Self = Self::new(VerbPresentStress::C, VerbPastStress::A);
    pub const C_A: Self = Self::new(VerbPresentStress::C, VerbPastStress::A);
    pub const C_B: Self = Self::new(VerbPresentStress::C, VerbPastStress::B);
    pub const C_C: Self = Self::new(VerbPresentStress::C, VerbPastStress::C);
    pub const C_Cp: Self = Self::new(VerbPresentStress::C, VerbPastStress::Cp);
    pub const C_Cpp: Self = Self::new(VerbPresentStress::C, VerbPastStress::Cpp);

    pub const Cp: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::A);
    pub const Cp_A: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::A);
    pub const Cp_B: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::B);
    pub const Cp_C: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::C);
    pub const Cp_Cp: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::Cp);
    pub const Cp_Cpp: Self = Self::new(VerbPresentStress::Cp, VerbPastStress::Cpp);
}
