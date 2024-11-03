use std::convert::Infallible;

use leptos::{Attribute, IntoAttribute};
use serde::Serialize;
use thiserror::Error;

use crate::impl_datatype;

use super::Datatype;

/// A datatype representing a non-empty string.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize)]
pub struct NonEmptyString(String);

/// The error type for the `NonEmptyString` datatype.
/// This error is returned when the input is an empty string and can be used to display an error message by providing a custom translation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum NonEmptyStringError {
    #[error("empty string")]
    EmptyString,
}

impl From<Infallible> for NonEmptyStringError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl Datatype for NonEmptyString {   
    type Inner = String;
    type Error = NonEmptyStringError;

    fn validate(input: String) -> Result<Self, NonEmptyStringError> {
        if input.is_empty() {
            Err(NonEmptyStringError::EmptyString)
        } else {
            Ok(Self(input))
        }
    }

    fn attributes() -> Vec<(&'static str, leptos::Attribute)> {
        vec![
            ("type", "text".into_attribute()),
            ("required", Attribute::Bool(true)),
        ]
    }

    fn default_debug_value() -> Self {
        Self::validate("Test".into()).unwrap()
    }
}

impl_datatype!(NonEmptyString);
