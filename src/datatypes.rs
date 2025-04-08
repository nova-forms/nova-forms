mod email;
mod non_empty_string;
mod phone;
mod date_time;
mod date;
mod time;
mod accept;
mod iban;
mod postal_code;
mod field;
mod regional_field;

pub use email::*;
pub use non_empty_string::*;
pub use phone::*;
pub use date_time::*;
pub use date::*;
pub use time::*;
pub use accept::*;
pub use iban::*;
pub use postal_code::*;
pub use field::*;
pub use regional_field::*;

use num_bigint::BigInt;
use num_rational::BigRational;

use leptos::*;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::rc::Rc;
use std::str::FromStr;

/// A datatype that represents an optional value.
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

macro_rules! impl_datatypes {
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

impl_datatypes! {
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

/// Implements the required traits for creating a datatype.
#[macro_export]
macro_rules! impl_datatype {
    ($this:ty) => {
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

    ($this:ty where $generic:ident : $sat:ident) => {
        impl<$generic>  std::str::FromStr for $this
        where
            $generic: $sat
        {
            type Err = <$this as $crate::Datatype>::Error;
        
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                <$this as $crate::Datatype>::validate(<$this as $crate::Datatype>::Inner::from_str(s).map_err(<$this as $crate::Datatype>::Error::from)?)
            }
        }

        impl<'de, $generic> serde::Deserialize<'de> for $this
        where
            $generic: $sat
        {
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

/// A trait for defining custom datatypes.
/// Implemented on all types that can be used as a form input.
pub trait Datatype: Clone + Display + Debug + Default + FromStr<Err = Self::Error> + Into<Self::Inner> + PartialEq + 'static {
    type Inner: Datatype;
    type Error: From<<Self::Inner as Datatype>::Error> + Error + Clone + PartialEq + 'static;

    /// Validate the input and return the datatype.
    fn validate(input: Self::Inner) -> Result<Self, Self::Error>
    where
        Self: Sized;

    /// Return the HTML attributes for the datatype that should be added to an input field.
    fn attributes() -> Vec<(&'static str, Attribute)>;
}

// Defines custom translations for a type `T`.
// This is useful for adding custom error messages to error enums.
#[derive(Clone)]
pub(crate) struct TranslationProvider<T>(Rc<dyn Fn(T) -> TextProp>);


/// Adds custom translations to a type `T`.
/// This is useful for adding custom error messages to error enums or other elements.
pub fn provide_translation<F, T>(f: F)
where
    T: Clone + 'static,
    F: Fn(T) -> TextProp + 'static,
{
    provide_context(TranslationProvider(Rc::new(f)));
}

pub fn expect_translation<T, F>(value: F) -> TextProp
where
    F: Into<MaybeSignal<T>>,
    T: Clone + 'static,
{
    let value = value.into();
    let translation_provider = expect_context::<TranslationProvider<T>>();
    (move || translation_provider.0(value.get()).get()).into()
}

pub fn use_translation<T, F>(value: F) -> TextProp
where
    F: Into<MaybeSignal<T>>,
    T: Clone + Display + 'static,
{
    let value = value.into();
    let translation_provider: Option<TranslationProvider<T>> = use_context::<TranslationProvider<T>>();
    if let Some(translation_provider) = translation_provider {
        (move || translation_provider.0(value.get()).get()).into()
    } else {
        (move || format!("{}", value.get())).into()
    }
}
