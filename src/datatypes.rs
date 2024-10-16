mod email;
mod non_empty_string;

pub use email::*;
pub use non_empty_string::*;

use leptos::{provide_context, Attribute, IntoView, Oco, View};
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;
use std::str::FromStr;

macro_rules! impl_primitive_datatypes {
    ( $( $t:ty where $($name:literal: $val:literal),* $(,)? );* $(;)? ) => {
        $(
            impl Datatype for $t {
                type Inner = $t;
                type Error = <$t as FromStr>::Err;

                fn validate(input: $t) -> Result<Self, <$t as FromStr>::Err> {
                    Ok(input)
                }

                fn attributes() -> Vec<(&'static str, Attribute)> {
                    vec![ $( ($name, Attribute::String(Oco::Borrowed($val))) ),* ]
                }
            }
        )*
    };
}



impl_primitive_datatypes! {
    u8 where "type": "number", "required": "", "step": "1", "min": "0", "max": "255";
    u16 where "type": "number", "required": "", "step": "1", "min": "0", "max": "65535";
    u32 where "type": "number", "required": "", "step": "1", "min": "0", "max": "4294967295";
    u64 where "type": "number", "required": "", "step": "1", "min": "0", "max": "18446744073709551615";
    u128 where "type": "number", "required": "", "step": "1", "min": "0", "max": "340282366920938463463374607431768211455";
    i8 where "type": "number", "required": "", "step": "1", "min": "-128", "max": "127";
    i16 where "type": "number", "required": "", "step": "1", "min": "-32768", "max": "32767";
    i32 where "type": "number", "required": "", "step": "1", "min": "-2147483648", "max": "2147483647";
    i64 where "type": "number", "required": "", "step": "1", "min": "-9223372036854775808", "max": "9223372036854775807";
    i128 where "type": "number", "required": "", "step": "1", "min": "-170141183460469231731687303715884105728", "max": "170141183460469231731687303715884105727";
    String where "type": "text";
}

/*
macro_rules! attrs {
    ($($k:literal $( = $v:literal )? )*) => {
        vec![
            $( ($k, {
                #[allow(unused_mut)]
                let mut v = Attribute::Bool(true);
                $( v = Attribute::String(Oco::Borrowed($v.to_string())); )?
                v
            }) ),*
        ]
    };
}
*/

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
