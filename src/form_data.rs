use std::{collections::HashMap, str::FromStr};

use leptos::*;
use percent_encoding::{percent_decode, percent_encode, NON_ALPHANUMERIC};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{QueryString, QueryStringPart};

/// Contains arbitrary form data in a serialized form.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FormData {
    data: RwSignal<HashMap<QueryString, String>>
}

impl FormData {
    pub fn serialize<F>(form_data: &F) -> Self
    where
        F: Serialize,
    {
        let serialized = serde_qs::to_string(form_data).expect("must be serializable");
        FormData::from_str(&serialized).unwrap()
    }

    pub fn deserialize<F>(&self) -> F
    where
        F: DeserializeOwned,
    {
        serde_qs::from_str(&self.to_string()).expect("must be deserializable")
    }
}

impl FromStr for FormData {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let map = s
            .split("&")
            .into_iter()
            .map(|pair| {
                pair.split_once("=")
                    .map(|(k, v)| {
                        (
                            QueryString::from(k),
                            percent_decode(v.as_bytes()).decode_utf8_lossy().to_string(),
                        )
                    })
                    .unwrap_or_else(|| (QueryString::from(pair), String::new()))
            })
            .collect();

        Ok(FormData {
            data: RwSignal::new(map)
        })
    }
}

impl ToString for FormData {
    fn to_string(&self) -> String {
        self.data
            .get()
            .iter()
            .map(|(k, v)| {
                format!("{}={}", 
                    k,
                    percent_encode(v.as_bytes(), NON_ALPHANUMERIC)
                )
            })
            .collect::<Vec<_>>()
            .join("&")
    }
}

impl FormData {
    pub fn values<T: DeserializeOwned + PartialEq>(&self, qs: QueryString) -> Signal<Option<T>> {
        #[derive(Deserialize)]
        struct Value<T> {
            value: T,
        }

        let signal = self.data;

        Memo::new(move |_| {
            let data = signal.get();

            let form_data = data
                .iter()
                .filter_map(|(k, v)| {
                    Some(format!("{}={}", 
                        QueryString::default().add_key("value").join(k.extends(&qs)?),
                        percent_encode(v.as_bytes(), NON_ALPHANUMERIC)
                    ))
                })
                .collect::<Vec<_>>()
                .join("&");
    
            Some(serde_qs::from_str::<Value<T>>(&form_data.to_string()).ok()?.value)
        }).into()
    }

    pub fn set_values<T: Serialize>(&self, qs: QueryString, values: T) {
        self.data.update(|data| {
            let form_data = FormData::serialize(&values);
            for (k, v) in form_data.data.get_untracked() {
                data.insert(qs.join(k), v);
            }
        });
    }

    pub fn raw_value(&self, qs: QueryString) -> Signal<String> {
        let signal = self.data;
        
        Memo::new(move |_| {
            let data = signal.get();

            let value = data
                .iter()
                .filter_map(|(k, v)| {
                    k.extends(&qs)?;
                    Some(v)
                })
                .next()
                .cloned()
                .unwrap_or_default();
    
            value
        }).into()
    }

    pub fn set_raw_value<S: Into<String>>(&self, qs: QueryString, value: S) {
        self.data.update(|data| {
            data.insert(qs.clone(), value.into());
        });
    }

    pub fn value<T: FromStr>(&self, qs: QueryString) -> Signal<Result<T, T::Err>> {
        let signal = self.raw_value(qs);
        Signal::derive(move || T::from_str(&signal.get()))
    }

    pub fn set_value<T: ToString>(&self, qs: QueryString, value: T) {
        self.set_raw_value(qs, value.to_string());
    }

    pub fn len(&self, qs: QueryString) -> Option<usize> {
        self.data
            .get_untracked()
            .keys()
            .map(|k| {
                if let Some(k) = k.extends(&qs) {
                    k.iter().next().and_then(|&p| {
                        if let QueryStringPart::Index(i) = p {
                            Some(i)
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            })
            .reduce(|l1, l2| {
                if let (Some(l1), Some(l2)) = (l1, l2) {
                    Some(l1.max(l2))
                } else {
                    None
                }
            })
            .flatten()
            .map(|l| l + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::qs;
    use super::*;

    #[test]
    fn test_value() {
        let _ = leptos::create_runtime();

        let form_data = FormData::from_str("a=1&b=2&c=3").unwrap();
        assert_eq!(form_data.value::<i32>(qs!(a)).get_untracked().unwrap(), 1);
        assert_eq!(form_data.value::<i32>(qs!(b)).get_untracked().unwrap(), 2);
        assert_eq!(form_data.value::<i32>(qs!(c)).get_untracked().unwrap(), 3);
    }

    #[test]
    fn test_set_value() {
        let _ = leptos::create_runtime();

        let form_data = FormData::from_str("a=1&b=2&c=3").unwrap();
        form_data.set_value(QueryString::from("a"), 4);
        assert_eq!(form_data.value::<i32>(qs!(a)).get_untracked().unwrap(), 4);
        assert_eq!(form_data.value::<i32>(qs!(b)).get_untracked().unwrap(), 2);
        assert_eq!(form_data.value::<i32>(qs!(c)).get_untracked().unwrap(), 3);
    }

    #[test]
    fn test_values() {
        let _ = leptos::create_runtime();

        #[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
        struct Test {
            a: i32,
            b: i32,
            c: i32,
        }

        let form_data = FormData::from_str("a=1&b=2&c=3").unwrap();
        assert_eq!(form_data.values::<Test>(qs!()).get_untracked().unwrap(), Test { a: 1, b: 2, c: 3 });
    }

    #[test]
    fn test_set_values() {
        let _ = leptos::create_runtime();

        #[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
        struct Test {
            a: i32,
            b: i32,
            c: i32,
        }

        let form_data = FormData::from_str("a=1&b=2&c=3").unwrap();
        form_data.set_values(qs!(), Test { a: 4, b: 5, c: 6 });
        assert_eq!(form_data.values::<Test>(qs!()).get_untracked().unwrap(), Test { a: 4, b: 5, c: 6 });
    }

    #[test]
    fn test_len() {
        let _ = leptos::create_runtime();

        let form_data = FormData::from_str("a[0]=1&a[3]=2&a[1]=3").unwrap();
        assert_eq!(form_data.len(qs!(a)).unwrap(), 4);
    }
}