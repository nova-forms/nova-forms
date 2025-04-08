use std::cell::LazyCell;
use regex::Regex;
use super::{Field, FieldError, Rule};

pub struct PostalCodeCHRule;

impl Rule for PostalCodeCHRule {
    const REGEX: LazyCell<Regex> =
        LazyCell::new(|| Regex::new(r"^[0-9]{4}$").unwrap());
    const DEFAULT: &'static str = "6210";
    const ATTRIBUTES: &'static [(&'static str, &'static str)] = &[("type", "text")];
}

pub type PostalCodeCH = Field<PostalCodeCHRule>;
pub type PostalCodeCHError = FieldError<PostalCodeCHRule>;