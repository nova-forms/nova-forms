use std::{cell::LazyCell, convert::Infallible};

use leptos::IntoAttribute;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;

use crate::impl_datatype;

use super::Datatype;

const EMAIL_REGEX: LazyCell<Regex> =
    LazyCell::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());

/// A datatype representing an email address.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize)]
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

impl Datatype for Email {   
    type Inner = String;
    type Error = EmailError;

    fn validate(input: String) -> Result<Email, EmailError> {
        if !EMAIL_REGEX.is_match(&input) {
            Err(EmailError::InvalidFormat)
        } else {
            Ok(Email(input))
        }
    }

    fn attributes() -> Vec<(&'static str, leptos::Attribute)> {
        vec![("type", "email".into_attribute())]
    }

    fn default_debug_value() -> Self {
        Self::validate("test@example.com".into()).unwrap()
    }
}

impl_datatype!(Email);
