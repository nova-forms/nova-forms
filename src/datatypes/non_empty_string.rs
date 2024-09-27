use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use serde::{de::Error, Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize)]
pub struct NonEmptyString(String);

impl Display for NonEmptyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<T> for NonEmptyString
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        NonEmptyString(value.into())
    }
}

impl Deref for NonEmptyString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NonEmptyString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'de> Deserialize<'de> for NonEmptyString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        if string.is_empty() {
            Err(D::Error::custom("empty_string"))
        } else {
            Ok(NonEmptyString(string))
        }
    }
}
