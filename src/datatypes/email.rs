use std::{cell::LazyCell, convert::Infallible};

use thiserror::Error;
use serde::{Deserialize, Serialize};
use regex::Regex;

use crate::custom_datatype;

const EMAIL_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
});

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct Email(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum EmailError {
    #[error("invalid format")]
    InvalidFormat,
}

impl From<Infallible> for EmailError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}


custom_datatype!{
    fn validate(input: String) -> Result<Email, EmailError> {
        if !EMAIL_REGEX.is_match(&input) {
            Err(EmailError::InvalidFormat)
        } else {
            Ok(Email(input))
        }
    }
}
