use serde::{Deserialize, Serialize};

use thiserror::Error;
use time::macros::format_description;
use time::UtcOffset;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

pub(crate) fn local_utc_offset() -> UtcOffset {
    if let Ok(offset) = UtcOffset::current_local_offset() {
        return offset;
    } else
    if cfg!(target_arch = "wasm32") {
        let offset_minutes = js_sys::Date::new_0().get_timezone_offset();
        UtcOffset::from_whole_seconds(offset_minutes as i32 * 60).unwrap()
    } else {
        UtcOffset::UTC
    }
}

/// A date and time.
/// **Important note**: This type does not support time zones.
/// To ensure that you are working with the clients timezone, you can
/// use the `local_utc_offset` field in the `MetaData` to translate this
/// `PrimitiveDateTime` to an `OffsetDateTime` with the correct offset.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime(time::PrimitiveDateTime);

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("{}", self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(DateTime::from_str(&s).map_err(serde::de::Error::custom)?)
    }
}

impl Default for DateTime {
    fn default() -> Self {
        let date_time = time::OffsetDateTime::now_utc().to_offset(local_utc_offset());
        DateTime(time::PrimitiveDateTime::new(date_time.date(), date_time.time()))
    }
}

impl Deref for DateTime {
    type Target = time::PrimitiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
    
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(format_description!("[year]-[month]-[day]T[hour]:[minute]")).unwrap())
    }
}

impl FromStr for DateTime {
    type Err = DateTimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DateTime(time::PrimitiveDateTime::parse(s, format_description!("[year]-[month]-[day]T[hour]:[minute]"))?))
    }
}

/// The error type for the `DateTime` datatype.
/// This error is returned when the input is not a valid date and time and can be used to display an error message by providing a custom translation.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DateTimeError {
    #[error(transparent)]
    Parse(#[from] time::error::Parse),
    #[error(transparent)]
    Offset(#[from] time::error::IndeterminateOffset),
}