use serde::{Deserialize, Serialize};
use time::macros::format_description;

use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

use super::DateTime;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time(time::Time);

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("{}", self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Time::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Default for Time {
    fn default() -> Self {
        Time(DateTime::default().time())
    }
}

impl Deref for Time {
    type Target = time::Time;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
    
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(format_description!("[hour]:[minute]")).unwrap())
    }
}

impl FromStr for Time {
    type Err = time::error::Parse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Time(time::Time::parse(s, format_description!("[hour]:[minute]"))?))
    }
}