use crate::{
    categories::{Case, CaseEx, Gender, GenderEx},
    util::enum_conversion,
};
use thiserror::Error;

#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error(
    "case must be one of the main 6: nominative, genitive, dative, accusative, instrumental or prepositional"
)]
pub struct CaseError;

#[derive(Debug, Default, Error, Clone, Copy, PartialEq, Eq)]
#[error("gender must be one of the main 3: masculine, neuter or feminine")]
pub struct GenderError;

enum_conversion! {
    Case => CaseEx { Nominative, Genitive, Dative, Accusative, Instrumental, Prepositional } else { CaseError }
}
enum_conversion! {
    Gender => GenderEx { Masculine, Neuter, Feminine } else { GenderError }
}
