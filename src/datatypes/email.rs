use std::cell::LazyCell;
use regex::Regex;
use super::{Field, FieldError, Rule};

pub struct EmailRule;

impl Rule for EmailRule {
    const REGEX: LazyCell<Regex> =
        LazyCell::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());
    const DEFAULT: &'static str = "test@example.com";
    const ATTRIBUTES: &'static [(&'static str, &'static str)] = &[("type", "email")];
}

pub type Email = Field<EmailRule>;
pub type EmailError = FieldError<EmailRule>;