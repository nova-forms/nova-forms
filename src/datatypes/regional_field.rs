
use std::{cell::LazyCell, convert::Infallible, fmt::{self, Display, Debug}, marker::PhantomData, ops::Deref, str::FromStr};

use leptos::IntoAttribute;
use regex::Regex;

use super::Datatype;

pub trait RegionalRule: 'static {
    const REGEX: LazyCell<Regex>;
    const DEFAULT: &'static str;
    const ATTRIBUTES: &'static [(&'static str, &'static str)] = &[("type", "text")];
}

pub struct RegionalField<R: RegionalRule> {
    value: String,
    _rule: PhantomData<R>,
}
pub enum RegionalFieldError<R: RegionalRule> {
    InvalidFormat(PhantomData<R>),
}

impl<R: RegionalRule> Debug for RegionalFieldError<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldError({:?})", self)
    }
}

impl<R: RegionalRule> Display for RegionalFieldError<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidFormat(_) => write!(f, "invalid format"),
        }
    }
}

impl<R: RegionalRule> std::error::Error for RegionalFieldError<R> {}

impl<R: RegionalRule> Clone for RegionalFieldError<R> {
    fn clone(&self) -> Self {
        match self {
            Self::InvalidFormat(_) => Self::InvalidFormat(PhantomData),
        }
    }
}

impl<R: RegionalRule> Copy for RegionalFieldError<R> {}

impl<R: RegionalRule> PartialEq for RegionalFieldError<R> {
    fn eq(&self, other: &Self) -> bool {
        matches!(self, Self::InvalidFormat(_)) && matches!(other, Self::InvalidFormat(_))
    }
}

impl<R: RegionalRule> Eq for RegionalFieldError<R> {}

impl<R: RegionalRule> From<Infallible> for RegionalFieldError<R> {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl<R: RegionalRule> Datatype for RegionalField<R> {
    type Inner = String;
    type Error = RegionalFieldError<R>;

    fn validate(input: String) -> Result<RegionalField<R>, RegionalFieldError<R>> {
        if !R::REGEX.is_match(&input) {
            Err(RegionalFieldError::InvalidFormat(PhantomData))
        } else {
            Ok(RegionalField {
                value: input,
                _rule: PhantomData,
            })
        }
    }

    fn attributes() -> Vec<(&'static str, leptos::Attribute)> {
        vec![("type", "text".into_attribute())]
    }
}

impl<R: RegionalRule> Clone for RegionalField<R> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _rule: PhantomData,
        }
    }
}

impl<R: RegionalRule> PartialEq for RegionalField<R> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<R: RegionalRule> Eq for RegionalField<R> {}

impl<R: RegionalRule> Default for RegionalField<R> {
    fn default() -> Self {
        Self::validate(R::DEFAULT.into()).unwrap()
    }
}

impl<R: RegionalRule> Display for RegionalField<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<R: RegionalRule> Debug for RegionalField<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Field({:?})", self.value)
    }
}

impl<R: RegionalRule> Deref for RegionalField<R> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<R: RegionalRule> Into<String> for RegionalField<R> {
    fn into(self) -> String {
        self.value
    }
}

impl<R: RegionalRule> FromStr for RegionalField<R> {
    type Err = <Self as Datatype>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as Datatype>::validate(<Self as Datatype>::Inner::from_str(s).map_err(<Self as Datatype>::Error::from)?)
    }
}

impl<'de, R: RegionalRule> serde::Deserialize<'de> for RegionalField<R> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <Self as Datatype>::Inner::deserialize(deserializer)?;
        Self::validate(value).map_err(serde::de::Error::custom)
    }
}