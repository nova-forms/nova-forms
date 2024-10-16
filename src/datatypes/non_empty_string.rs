use std::convert::Infallible;

use serde::Serialize;
use thiserror::Error;

use crate::custom_datatype;

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

custom_datatype! {
    fn validate(input: String) -> Result<NonEmptyString, NonEmptyStringError> {
        if input.is_empty() {
            Err(NonEmptyStringError::EmptyString)
        } else {
            Ok(NonEmptyString(input))
        }
    }
}
