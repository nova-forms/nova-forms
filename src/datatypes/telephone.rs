use std::{cell::LazyCell, convert::Infallible};

use leptos::IntoAttribute;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;

use crate::impl_datatype;

use super::Datatype;

const EMAIL_REGEX: LazyCell<Regex> =
    LazyCell::new(|| Regex::new(r"^[\+]?[(]?[0-9]{3}[)]?[-\s\.]?[0-9]{3}[-\s\.]?[0-9]{4,6}$").unwrap());

/// A datatype representing a telephone number.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize)]
pub struct Telephone(String);

/// The error type for the `Telephone` datatype.
/// This error is returned when the input is not a valid telephone number and can be used to display an error message by providing a custom translation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum TelephoneError {
    #[error("invalid format")]
    InvalidFormat,
}

impl From<Infallible> for TelephoneError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl Datatype for Telephone {   
    type Inner = String;
    type Error = TelephoneError;

    fn validate(input: String) -> Result<Telephone, TelephoneError> {
        if !EMAIL_REGEX.is_match(&input) {
            Err(TelephoneError::InvalidFormat)
        } else {
            Ok(Telephone(input))
        }
    }

    fn attributes() -> Vec<(&'static str, leptos::Attribute)> {
        vec![("type", "tel".into_attribute())]
    }

    fn default_debug_value() -> Self {
        Self::validate("+41234567890".into()).unwrap()
    }
}

impl_datatype!(Telephone);
