use std::convert::Infallible;

use serde::Serialize;
use thiserror::Error;

use crate::impl_custom_datatype;

use super::CustomDatatype;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize)]
pub struct NonEmptyString(String);

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

impl CustomDatatype for NonEmptyString {   
    type Inner = String;
    type Error = NonEmptyStringError;

    fn validate(input: String) -> Result<Self, NonEmptyStringError> {
        if input.is_empty() {
            Err(NonEmptyStringError::EmptyString)
        } else {
            Ok(Self(input))
        }
    }
}

impl_custom_datatype!(NonEmptyString);
