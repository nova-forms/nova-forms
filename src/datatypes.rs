mod email;
mod non_empty_string;
mod telephone;
mod date_time;
mod date;
mod time;

pub use email::*;
pub use non_empty_string::*;
pub use telephone::*;
pub use date_time::*;
pub use date::*;
pub use time::*;

use num_bigint::BigInt;
use num_rational::BigRational;

use leptos::{provide_context, Attribute, IntoView, Oco, View};
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Optional<T>(Option<T>);

impl<T> FromStr for Optional<T>
where
    T: FromStr,
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Optional(None));
        }
        Ok(Optional(T::from_str(s).ok()))
    }
}

impl<T> Display for Optional<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(value) = &self.0 {
            write!(f, "{}", value)
        } else {
            write!(f, "")
        }
    }
}

macro_rules! impl_direct_datatypes {
    ( $( $t:ty where $($name:literal $( : $val:literal)? ),* $(,)? );* $(;)? ) => {
        $(
            impl Datatype for $t {
                type Inner = $t;
                type Error = <$t as FromStr>::Err;

                fn validate(input: $t) -> Result<Self, <$t as FromStr>::Err> {
                    Ok(input)
                }

                fn attributes() -> Vec<(&'static str, Attribute)> {
                    vec![ $( ($name, {
                        #[allow(unused)]
                        let mut v = Attribute::Bool(true);
                        $( v = Attribute::String(Oco::Borrowed($val)); )?
                        v
                    }) ),* ]
                }
            }
        )*
    };
}

impl_direct_datatypes! {
    u8 where "type": "number", "step": "1", "min": "0", "max": "255", "required";
    Optional<u8> where "type": "number", "step": "1", "min": "0", "max": "255";
    u16 where "type": "number", "step": "1", "min": "0", "max": "65535", "required";
    Optional<u16> where "type": "number", "step": "1", "min": "0", "max": "65535";
    u32 where "type": "number", "step": "1", "min": "0", "max": "4294967295", "required";
    Optional<u32> where "type": "number", "step": "1", "min": "0", "max": "4294967295";
    u64 where "type": "number", "step": "1", "min": "0", "max": "18446744073709551615", "required";
    Optional<u64> where "type": "number", "step": "1", "min": "0", "max": "18446744073709551615";
    u128 where "type": "number", "step": "1", "min": "0", "max": "340282366920938463463374607431768211455", "required";
    Optional<u128> where "type": "number", "step": "1", "min": "0", "max": "340282366920938463463374607431768211455";
    i8 where "type": "number", "step": "1", "min": "-128", "max": "127", "required";
    Optional<i8> where "type": "number", "step": "1", "min": "-128", "max": "127";
    i16 where "type": "number", "step": "1", "min": "-32768", "max": "32767", "required";
    Optional<i16> where "type": "number", "step": "1", "min": "-32768", "max": "32767";
    i32 where "type": "number", "step": "1", "min": "-2147483648", "max": "2147483647", "required";
    Optional<i32> where "type": "number", "step": "1", "min": "-2147483648", "max": "2147483647";
    i64 where "type": "number", "step": "1", "min": "-9223372036854775808", "max": "9223372036854775807", "required";
    Optional<i64> where "type": "number", "step": "1", "min": "-9223372036854775808", "max": "9223372036854775807";
    i128 where "type": "number", "step": "1", "min": "-170141183460469231731687303715884105728", "max": "170141183460469231731687303715884105727", "required";
    Optional<i128> where "type": "number", "step": "1", "min": "-170141183460469231731687303715884105728", "max": "170141183460469231731687303715884105727";
    String where "type": "text";
    BigInt where "type": "number", "step": "1", "required";
    Optional<BigInt> where "type": "number", "step": "1";
    BigRational where "type": "number", "required";
    Optional<BigRational> where "type": "number";
    DateTime where "type": "datetime-local", "required";
    Optional<DateTime> where "type": "datetime-local";
    Date where "type": "date", "required";
    Optional<Date> where "type": "date";
    Time where "type": "time", "required";
    Optional<Time> where "type": "time";
    bool where "type": "checkbox";
}

#[macro_export]
macro_rules! impl_datatype {
    ($this:ty) => {
        impl std::fmt::Display for $this {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::ops::Deref for $this {
            type Target = String;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::str::FromStr for $this {
            type Err = <$this as $crate::Datatype>::Error;
        
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                <$this as $crate::Datatype>::validate(<$this as $crate::Datatype>::Inner::from_str(s).map_err(<$this as $crate::Datatype>::Error::from)?)
            }
        }

        impl<'de> serde::Deserialize<'de> for $this {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = <$this as $crate::Datatype>::Inner::deserialize(deserializer)?;
                <$this as $crate::Datatype>::validate(value).map_err(serde::de::Error::custom)
            }
        }
    };
}

pub trait Datatype: Clone + Display + FromStr<Err = Self::Error> + 'static {
    type Inner: Datatype;
    type Error: From<<Self::Inner as Datatype>::Error> + Error + Clone + 'static;

    fn validate(input: Self::Inner) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn attributes() -> Vec<(&'static str, Attribute)>;
}

#[derive(Clone)]
pub struct TranslationProvider<T>(Rc<dyn Fn(T) -> View>);

impl<T> TranslationProvider<T> {
    pub fn t(&self, value: T) -> View {
        (self.0)(value)
    }
}

impl<T, F> From<F> for TranslationProvider<T>
where
    F: Fn(T) -> View + 'static,
{
    fn from(f: F) -> Self {
        TranslationProvider(Rc::new(f))
    }
}

pub fn provide_translation_context<F, T, V>(f: F)
where
    T: Clone + 'static,
    V: IntoView + 'static,
    F: Fn(T) -> V + Clone + 'static,
{
    provide_context(TranslationProvider::from(move |value| f(value).into_view()));
}
