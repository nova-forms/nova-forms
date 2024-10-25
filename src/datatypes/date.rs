use serde::{Deserialize, Serialize};
use time::macros::format_description;

use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

use super::DateTime;


/// A date without a time.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date(time::Date);

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("{}", self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Date::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Default for Date {
    fn default() -> Self {
        Date(DateTime::default().date())
    }
}

impl Deref for Date {
    type Target = time::Date;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
    
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(format_description!("[year]-[month]-[day]")).unwrap())
    }
}

impl FromStr for Date {
    type Err = time::error::Parse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Date(time::Date::parse(s, format_description!("[year]-[month]-[day]"))?))
    }
}