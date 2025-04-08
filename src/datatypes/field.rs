
use std::{cell::LazyCell, convert::Infallible, fmt::{self, Display, Debug}, marker::PhantomData, ops::Deref, str::FromStr};

use leptos::IntoAttribute;
use regex::Regex;

use super::Datatype;

pub trait Rule: 'static {
    const REGEX: LazyCell<Regex>;
    const DEFAULT: &'static str;
    const ATTRIBUTES: &'static [(&'static str, &'static str)] = &[("type", "text")];
}

pub struct Field<R: Rule> {
    value: String,
    _rule: PhantomData<R>,
}

pub enum FieldError<R: Rule> {
    InvalidFormat(PhantomData<R>),
}

impl<R: Rule> Debug for FieldError<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldError({:?})", self)
    }
}

impl<R: Rule> Display for FieldError<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidFormat(_) => write!(f, "invalid format"),
        }
    }
}

impl<R: Rule> std::error::Error for FieldError<R> {}

impl<R: Rule> Clone for FieldError<R> {
    fn clone(&self) -> Self {
        match self {
            Self::InvalidFormat(_) => Self::InvalidFormat(PhantomData),
        }
    }
}

impl<R: Rule> Copy for FieldError<R> {}

impl<R: Rule> PartialEq for FieldError<R> {
    fn eq(&self, other: &Self) -> bool {
        matches!(self, Self::InvalidFormat(_)) && matches!(other, Self::InvalidFormat(_))
    }
}

impl<R: Rule> Eq for FieldError<R> {}

impl<R: Rule> From<Infallible> for FieldError<R> {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl<R: Rule> Datatype for Field<R> {
    type Inner = String;
    type Error = FieldError<R>;

    fn validate(input: String) -> Result<Field<R>, FieldError<R>> {
        if !R::REGEX.is_match(&input) {
            Err(FieldError::InvalidFormat(PhantomData))
        } else {
            Ok(Field {
                value: input,
                _rule: PhantomData,
            })
        }
    }

    fn attributes() -> Vec<(&'static str, leptos::Attribute)> {
        vec![("type", "text".into_attribute())]
    }
}

impl<R: Rule> Clone for Field<R> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _rule: PhantomData,
        }
    }
}

impl<R: Rule> PartialEq for Field<R> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<R: Rule> Eq for Field<R> {}

impl<R: Rule> Default for Field<R> {
    fn default() -> Self {
        Self::validate(R::DEFAULT.into()).unwrap()
    }
}

impl<R: Rule> Display for Field<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<R: Rule> Debug for Field<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Field({:?})", self.value)
    }
}

impl<R: Rule> Deref for Field<R> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<R: Rule> Into<String> for Field<R> {
    fn into(self) -> String {
        self.value
    }
}

impl<R: Rule> FromStr for Field<R> {
    type Err = <Self as Datatype>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as Datatype>::validate(<Self as Datatype>::Inner::from_str(s).map_err(<Self as Datatype>::Error::from)?)
    }
}

impl<'de, R: Rule> serde::Deserialize<'de> for Field<R> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <Self as Datatype>::Inner::deserialize(deserializer)?;
        Self::validate(value).map_err(serde::de::Error::custom)
    }
}

impl<R: Rule> serde::Serialize for Field<R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}