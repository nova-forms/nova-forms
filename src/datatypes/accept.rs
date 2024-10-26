use std::str::ParseBoolError;

use leptos::IntoAttribute;
use serde::Serialize;
use thiserror::Error;

use crate::impl_datatype;

use super::Datatype;

/// A datatype for checkboxes that require them to be true.
/// This is useful for a checkbox that needs to be checked before submitting a form,
/// for example a terms and conditions checkbox. 
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize)]
pub struct Accept(bool);

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

    fn default_debug_value() -> Self {
        Self::validate(true.into()).unwrap()
    }
}

impl_datatype!(Accept);
