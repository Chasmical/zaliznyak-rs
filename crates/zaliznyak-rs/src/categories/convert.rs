use crate::{
    categories::{Case, CaseEx, Gender, GenderEx},
    util::enum_conversion,
};
use thiserror::Error;

/// The error type for conversion from [`CaseEx`] to [`Case`]. Possibly returned from [`Case::try_from`].
#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
#[error(
    "case must be one of the main 6: nominative, genitive, dative, accusative, instrumental or prepositional"
)]
pub struct CaseError;

/// The error type for conversion from [`GenderEx`] to [`Gender`]. Possibly returned from [`Gender::try_from`].
#[derive(Debug, Error, Copy, Eq, Hash)]
#[derive_const(Default, Clone, PartialEq)]
#[error("gender must be one of the main 3: masculine, neuter or feminine")]
pub struct GenderError;

enum_conversion! {
    Case => CaseEx { Nominative, Genitive, Dative, Accusative, Instrumental, Prepositional } else { CaseError }
}
enum_conversion! {
    Gender => GenderEx { Masculine, Neuter, Feminine } else { GenderError }
}
