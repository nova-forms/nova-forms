mod non_empty_string;
mod email;

pub use non_empty_string::*;
pub use email::*;

use leptos::{Attribute, Oco, View};
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;
use std::str::FromStr;

pub trait Datatype: Display + Default + Clone + FromStr<Err = Self::Error> + 'static {
    type Error: Error + Clone + 'static;

    fn attributes() -> Vec<(&'static str, Attribute)> {
        Vec::new()
    }
}

macro_rules! datatype {
    ( $( $t:ty => $($name:literal: $val:literal),* $(,)? );* $(;)? ) => {
        $(
            impl Datatype for $t { 
                type Error = <$t as FromStr>::Err;

                fn attributes() -> Vec<(&'static str, Attribute)> {
                    vec![ $( ($name, Attribute::String(Oco::Borrowed($val))) ),* ]
                }
            }
        )*
    };
}

datatype! {
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
}

#[macro_export]
macro_rules! custom_datatype {
    (fn validate($var:ident : $inner:ty) -> Result<$this:ty, $err:ty> { $($body:tt)* } ) => {
        impl std::fmt::Display for $this {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::str::FromStr for $this {
            type Err = $err;
        
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                <Self as $crate::CustomDatatype>::validate(<Self as $crate::CustomDatatype>::Inner::from_str(s).map_err(<Self as $crate::Datatype>::Error::from)?)
            }
        }
        
        impl TryFrom<String> for $this {
            type Error = $err;
        
            fn try_from(value: String) -> Result<Self, Self::Error> {
                <Self as $crate::CustomDatatype>::validate(value)
            }
        }
        
        impl std::ops::Deref for $this {
            type Target = String;
        
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        
        impl $crate::CustomDatatype for $this {
            type Inner = $inner;
            type Error =  $err;
        
            fn validate($var: Self::Inner) -> Result<Self, <Self as $crate::CustomDatatype>::Error> {
                $($body)*
            }
        }
    };
}


pub trait CustomDatatype: Datatype<Error = <Self as CustomDatatype>::Error> {
    type Inner: Datatype;
    type Error: From<<Self::Inner as Datatype>::Error> + Error + Clone + 'static;

    fn validate(input: Self::Inner) -> Result<Self, <Self as CustomDatatype>::Error> where Self: Sized;
}

impl<T> Datatype for T
where
    T: CustomDatatype,
    T::Inner: Datatype,
{
    type Error =<Self as CustomDatatype>::Error;

    fn attributes() -> Vec<(&'static str, Attribute)> {
        T::Inner::attributes()
    }
}

#[derive(Clone)]
pub struct Translate<T>(Rc<dyn Fn(T) -> View>);

impl<T> Translate<T> {
    pub fn t(&self, value: T) -> View {
        (self.0)(value)
    }
}

impl<T, F> From<F> for Translate<T>
where
    F: Fn(T) -> View + 'static,
{
    fn from(f: F) -> Self {
        Translate(Rc::new(f))
    }
}
