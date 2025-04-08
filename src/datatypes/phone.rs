use std::cell::LazyCell;
use regex::Regex;
use super::{Field, FieldError, Rule};

pub struct PhoneRule;

impl Rule for PhoneRule {
    const REGEX: LazyCell<Regex> =
        LazyCell::new(|| Regex::new(r"^\+?[0-9]{1,3}[0-9]{3,14}$").unwrap());
    const DEFAULT: &'static str = "+41791234567";
    const ATTRIBUTES: &'static [(&'static str, &'static str)] = &[("type", "phone")];
}

pub type Phone = Field<PhoneRule>;
pub type PhoneError = FieldError<PhoneRule>;