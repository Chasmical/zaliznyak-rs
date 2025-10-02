//! Grammatical categories (case, number, gender, animacy, etc.).
//!
//! # Grammatical number
//!
//! Just like English, Russian has [`Singular`] and [`Plural`] numbers, as well as singulare
//! tantum and plurale tantum (words that have only one grammatical number, such as "emptiness"
//! and "scissors" in English, and "сено" (hay) and "ножницы" (scissors) in Russian).
//!
//! In English you can pluralize a word simply by appending **s** or **es** to it (**plane** ---
//! **planes**, **box** --- **boxes**), and sometimes transforming a few last letters (e.g.
//! **lily** --- **lilies**). Russian mostly follows a similar scheme, --- appending **ы**, **и**,
//! **а** or **я** to the word's stem. You can somewhat reliably pick the correct ending based on
//! the word's last letter, but even then, there are numerous exceptions to this rule, even in the
//! simplest of words: **сон** (dream) --- **сны** (middle **о** disappears), **глаз** (eye) ---
//! **глаза** (**а** instead of the usual **ы** after **з**).
//!
//! # Grammatical case
//!
//! English only uses cases with pronouns: subjective ("I"), objective ("me"), reflexive ("myself"),
//! possessive ("my"/"mine"). It's the same pronoun, just declined differently. All the other words
//! don't use cases at all.
//!
//! Russian applies cases to **almost all words** (nouns, adjectives and pronouns), and there are
//! at least 6 of those cases: [`Nominative`] ("X is"), [`Genitive`] ("from X"), [`Dative`]
//! ("to X"), [`Accusative`] ("see X"), [`Instrumental`] ("do sth using X"), [`Prepositional`]
//! ("about X"). There are also *at least* 3 rare secondary cases: [`Partitive`] ("of X"),
//! [`Translative`] ("into X"), [`Locative`] ("in X").
//!
//! Most of the time, a case simply gives the word a different ending: **рука** (hand), **руки**,
//! **руке**, **руку**, **рукой**, **руке**. But, just like with grammatical numbers, the word's
//! stem is often altered unpredictably: **боец** (fighter), **бойца**, **бойцу**, **бойцом**,
//! **бойце**.
//!
//! # Grammatical gender
//!
//! All Russian nouns have a gender, that dictates how the word is declined, what endings it has,
//! and what gender adjectives referring to it have. There are three: [`Masculine`], [`Feminine`]
//! and [`Neuter`] ("middle", or inanimate) genders. There's also a relatively rare [`Common`]
//! gender that depends on the gender of the person it refers to, but it's morphologically
//! identical to feminine, so essentially there are just three.
//!
//! Russian applies genders to **all words**: nouns, adjectives, pronouns, and even verbs. Also,
//! sometimes the noun's morphological gender is different from its agreed gender. For example,
//! the word "мужчина" (man) is a masculine noun, but uses feminine endings, and the adjectives
//! that refer to it still treat it as masculine and use masculine endings. (e.g. dative case:
//! **красивому** (masc.) **мужчине** (fem.))
//!
//! # Grammatical animacy
//!
//! All Russian nouns also have animacy ([`Animate`] or [`Inanimate`]), indicating whether the
//! objects they're referring to are considered "alive/sentient", or not. The animacy affects the
//! [`Accusative`] case's endings for [`Masculine`], [`Neuter`] and [`Plural`] forms: if it's
//! animate then it uses [`Genitive`]'s endings, and if it's inanimate --- [`Nominative`]'s.
//!
//! # Inflection parameters
//!
//! For the ease of passing around all the parameters, there are two structures combining some of
//! the enums: [`DeclInfo`] for declension (nouns, adjectives, pronouns) and [`ConjInfo`] for
//! conjugation (verbs).
//!
//! ```
//! // FIXME: When AdjectiveInfo parsing is implemented, simplify this example
//!
//! use zaliznyak::{
//!     adjective::{Adjective, AdjectiveFlags, AdjectiveInfo, AdjectiveKind},
//!     categories::{Animacy, Case, DeclInfo, Gender, Number},
//!     declension::{AdjectiveDeclension, AdjectiveStemType, Declension, DeclensionFlags},
//!     stress::AdjectiveStress,
//! };
//!
//! let adj = Adjective::from_word("надёжный", AdjectiveInfo {
//!     kind: AdjectiveKind::Regular,
//!     flags: AdjectiveFlags::empty(),
//!     declension: Some(Declension::Adjective(AdjectiveDeclension {
//!         stem_type: AdjectiveStemType::Type1,
//!         stress: AdjectiveStress::A,
//!         flags: DeclensionFlags::STAR,
//!     })),
//! }).unwrap();
//!
//! let info = DeclInfo {
//!     case: Case::Instrumental,
//!     number: Number::Plural,
//!     gender: Gender::Feminine,
//!     animacy: Animacy::Inanimate,
//! };
//!
//! assert_eq!(adj.inflect(info).as_str(), "надёжными");
//! ```
//!
//! [`Singular`]: Number::Singular
//! [`Plural`]: Number::Plural
//! [`Nominative`]: Case::Nominative
//! [`Genitive`]: Case::Genitive
//! [`Dative`]: Case::Dative
//! [`Accusative`]: Case::Accusative
//! [`Instrumental`]: Case::Instrumental
//! [`Prepositional`]: Case::Prepositional
//! [`Partitive`]: CaseEx::Partitive
//! [`Translative`]: CaseEx::Translative
//! [`Locative`]: CaseEx::Locative
//! [`Masculine`]: Gender::Masculine
//! [`Neuter`]: Gender::Neuter
//! [`Feminine`]: Gender::Feminine
//! [`Common`]: GenderEx::Common
//! [`Animate`]: Animacy::Animate
//! [`Inanimate`]: Animacy::Inanimate

mod abbrs;
mod convert;
mod info;
mod methods;
mod traits;

pub use convert::*;
pub use info::*;
pub use traits::*;

/// One of the 6 primary grammatical cases (see [`Case`]) or 3 secondary cases.
///
/// See the [module-level documentation][self] for more details.
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
#[non_exhaustive]
pub enum CaseEx {
    /// Nominative case (who? what?). Именительный падеж (кто? что?).
    #[default]
    Nominative = 0,
    /// Genitive case (whose? from who? what of?). Родительный падеж (кого? чего?).
    Genitive = 1,
    /// Dative case (to who? to what?). Дательный падеж (кому? чему?).
    Dative = 2,
    /// Accusative case (whom? what?). Винительный падеж (кого? что?).
    Accusative = 3,
    /// Instrumental case (how? with who? with what?). Творительный падеж (кем? чем?).
    Instrumental = 4,
    /// Prepositional case (about who? about what?). Предложный падеж (о ком? о чём?).
    Prepositional = 5,
    /// Partitive case (of what?). Частичный падеж (чего?).
    ///
    /// Also known as second genitive (второй родительный).
    Partitive = 6,
    /// Translative case (into who? plural). Превратительный падеж (в кого? мн.ч.).
    ///
    /// Also known as second accusative (второй винительный).
    Translative = 7,
    /// Locative case (where? inside of what?). Местный падеж (где? в чём?).
    ///
    /// Also known as second prepositional (второй предложный).
    Locative = 8,
}
/// One of the 6 primary grammatical cases used in standard declension.
///
/// See the [module-level documentation][self] for more details.
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub enum Case {
    /// Nominative case (who? what?). Именительный падеж (кто? что?).
    #[default]
    Nominative = 0,
    /// Genitive case (whose? from who? what of?). Родительный падеж (кого? чего?).
    Genitive = 1,
    /// Dative case (to who? to what?). Дательный падеж (кому? чему?).
    Dative = 2,
    /// Accusative case (whom? what?). Винительный падеж (кого? что?).
    Accusative = 3,
    /// Instrumental case (how? with who? with what?). Творительный падеж (кем? чем?).
    Instrumental = 4,
    /// Prepositional case (about who? about what?). Предложный падеж (о ком? о чём?).
    Prepositional = 5,
}

/// One of the 3 primary grammatical genders (see [`Gender`]), or [`Common`][GenderEx::Common] gender.
///
/// See the [module-level documentation][self] for more details.
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub enum GenderEx {
    /// Masculine gender (he). Мужской род (он).
    #[default]
    Masculine = 0,
    /// Neuter gender (it). Средний род (оно).
    Neuter = 1,
    /// Feminine gender (she). Женский род (она).
    Feminine = 2,
    /// Common gender (he/she). Общий род (он/она).
    Common = 3,
}
/// One of the 3 primary grammatical genders used in standard inflection.
///
/// See the [module-level documentation][self] for more details.
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub enum Gender {
    /// Masculine gender (he). Мужской род (он).
    #[default]
    Masculine = 0,
    /// Neuter gender (it). Средний род (оно).
    Neuter = 1,
    /// Feminine gender (she). Женский род (она).
    Feminine = 2,
}

/// Grammatical animacy.
///
/// See the [module-level documentation][self] for more details.
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub enum Animacy {
    /// Inanimate. Неодушевлённое.
    #[default]
    Inanimate = 0,
    /// Animate. Одушевлённое.
    Animate = 1,
}
/// Grammatical number.
///
/// See the [module-level documentation][self] for more details.
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub enum Number {
    /// Singular number. Единственное число.
    #[default]
    Singular = 0,
    /// Plural number. Множественное число.
    Plural = 1,
}

/// Grammatical tense.
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub enum Tense {
    /// Present (is doing) or future tense (will do), depending on verb's aspect.
    ///
    /// Настоящее (делаю) или будущее время (сделаю), в зависимости от совершенности глагола.
    #[default]
    Present,
    /// Past tense (was doing/did). Прошедшее время (делал/сделал).
    Past,
}
/// Grammatical person.
#[derive(Debug, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
pub enum Person {
    /// First person (I/we). Первое лицо (я/мы).
    #[default]
    First,
    /// Second person (you). Второе лицо (ты/вы).
    Second,
    /// Third person (he/it/she/they). Третье лицо (он/оно/она/они).
    Third,
}

impl CaseEx {
    pub const VALUES: [Self; 9] = [
        Self::Nominative,
        Self::Genitive,
        Self::Dative,
        Self::Accusative,
        Self::Instrumental,
        Self::Prepositional,
        Self::Partitive,
        Self::Translative,
        Self::Locative,
    ];
}
impl Case {
    pub const VALUES: [Self; 6] = [
        Self::Nominative,
        Self::Genitive,
        Self::Dative,
        Self::Accusative,
        Self::Instrumental,
        Self::Prepositional,
    ];
}

impl GenderEx {
    pub const VALUES: [Self; 4] = [Self::Masculine, Self::Neuter, Self::Feminine, Self::Common];
}
impl Gender {
    pub const VALUES: [Self; 3] = [Self::Masculine, Self::Neuter, Self::Feminine];
}

impl Animacy {
    pub const VALUES: [Self; 2] = [Self::Inanimate, Self::Animate];
}
impl Number {
    pub const VALUES: [Self; 2] = [Self::Singular, Self::Plural];
}
