mod non_empty_string;

use leptos::{Attribute, Oco};
pub use non_empty_string::*;

use serde::{de::DeserializeOwned, Deserialize};
use std::fmt::Display;

pub trait Datatype: Display + DeserializeOwned + Default + Clone + 'static {
    fn validate<S: AsRef<str>>(input: S) -> Result<Self, String> {
        #[derive(Deserialize)]
        struct Wrapper<T> {
            value: T,
        }

        serde_qs::from_str::<'_, Wrapper<Self>>(&format!("value={}", input.as_ref()))
            .map(|wrapper| wrapper.value)
            .map_err(|err| format!("{err}"))
    }

    fn attributes() -> Vec<(&'static str, Attribute)> {
        Vec::new()
    }
}

macro_rules! impl_datatype {
    ( $( $t:ty => $($name:literal: $val:literal),* $(,)? );* $(;)? ) => {
        $(
            impl Datatype for $t {
                fn attributes() -> Vec<(&'static str, Attribute)> {
                    vec![ $( ($name, Attribute::String(Oco::Borrowed($val))) ),* ]
                }
            }
        )*
    };
}

impl_datatype! {
    u8 => "type": "number", "step": "1", "min": "0", "max": "255";
    u16 => "type": "number", "step": "1", "min": "0", "max": "65535";
    u32 => "type": "number", "step": "1", "min": "0", "max": "4294967295";
    u64 => "type": "number", "step": "1", "min": "0", "max": "18446744073709551615";
    u128 => "type": "number", "step": "1", "min": "0", "max": "340282366920938463463374607431768211455";
    i8 => "type": "number", "step": "1", "min": "-128", "max": "127";
    i16 => "type": "number", "step": "1", "min": "-32768", "max": "32767";
    i32 => "type": "number", "step": "1", "min": "-2147483648", "max": "2147483647";
    i64 => "type": "number", "step": "1", "min": "-9223372036854775808", "max": "9223372036854775807";
    i128 => "type": "number", "step": "1", "min": "-170141183460469231731687303715884105728", "max": "170141183460469231731687303715884105727";
    String => "type": "text";
    NonEmptyString => "type": "text", "minlength": "1";
}
