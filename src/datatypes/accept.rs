use std::{fmt::{Display, self}, ops::Deref, str::ParseBoolError};

use leptos::IntoAttribute;
use serde::Serialize;
use thiserror::Error;

use crate::impl_datatype;

use super::Datatype;

/// A datatype for checkboxes that require them to be true.
/// This is useful for a checkbox that needs to be checked before submitting a form,
/// for example a terms and conditions checkbox. 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Accept(bool);

/// The error type for the `Accept` datatype.
/// This error is returned when the input is not accepted and can be used to display an error message by providing a custom translation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum AcceptError {
    #[error("not accepted")]
    NotAccepted,
}

impl From<ParseBoolError> for AcceptError {
    fn from(_: ParseBoolError) -> Self {
        Self::NotAccepted
    }
}

impl Datatype for Accept {   
    type Inner = bool;
    type Error = AcceptError;

    fn validate(input: bool) -> Result<Accept, AcceptError> {
        if !input {
            Err(AcceptError::NotAccepted)
        } else {
            Ok(Accept(input))
        }
    }

    fn attributes() -> Vec<(&'static str, leptos::Attribute)> {
        vec![("type", "checkbox".into_attribute())]
    }
}

impl Default for Accept {
    fn default() -> Self {
        Self::validate(true.into()).unwrap()
    }
}

impl Display for Accept {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Accept {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<bool> for Accept {
    fn into(self) -> bool {
        self.0
    }
}

impl_datatype!(Accept);
