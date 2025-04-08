use std::cell::LazyCell;
use regex::Regex;
use super::{Field, FieldError, Rule};

pub struct IbanRule;

impl Rule for IbanRule {
    const REGEX: LazyCell<Regex> =
        LazyCell::new(|| Regex::new(r"^[A-Z]{2}[0-9]{2}[a-zA-Z0-9]{11,30}$").unwrap());
    const DEFAULT: &'static str = "CH9300762011623852957";
    const ATTRIBUTES: &'static [(&'static str, &'static str)] = &[("type", "text")];
}

pub type Iban = Field<IbanRule>;
pub type IbanError = FieldError<IbanRule>;