use std::{collections::HashMap, fmt::Display};

use leptos::{expect_context, Attribute, IntoAttribute, Oco};
use serde::Serialize;

use crate::Datatype;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum QueryStringPart {
    Index(usize),
    Key(String),
}

impl Display for QueryStringPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryStringPart::Index(i) => write!(f, "{}", i),
            QueryStringPart::Key(k) => write!(f, "{}", k),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct QueryString(Vec<QueryStringPart>);

impl QueryString {
    fn extends(&self, other: &Self) -> Option<QueryString> {
        if self.0.len() < other.0.len() {
            return None;
        }

        if !self.0.iter().zip(other.0.iter()).all(|(s, o)| s == o) {
            return None;
        }

        Some(QueryString(
            self.0.iter().skip(other.0.len()).cloned().collect(),
        ))
    }

    pub fn form_context(&self) -> (QueryString, FormDataSerialized) {
        let form_data = expect_context::<FormDataSerialized>();
        let curr_form_data = form_data.level(&self);
        let prefix_qs = expect_context::<QueryString>();
        let curr_qs = prefix_qs.join(self.clone());
        (curr_qs, curr_form_data)
    }

    pub fn form_value<T: Datatype>(&self) -> (QueryString, Result<T, String>) {
        let form_data = expect_context::<FormDataSerialized>();
        let value = form_data
            .exact(&self)
            .map(|value| T::validate(value))
            .unwrap_or_else(|| Ok(T::default()));
        let prefix_qs = expect_context::<QueryString>();
        let curr_qs = prefix_qs.join(self.clone());
        (curr_qs, value)
    }
}

impl IntoAttribute for QueryString {
    fn into_attribute(self) -> Attribute {
        Attribute::String(Oco::Owned(format!("{self}")))
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        Attribute::String(Oco::Owned(format!("{self}")))
    }
}

impl From<&str> for QueryString {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        let mut parts = Vec::new();
        while let Some(c) = chars.next() {
            match c {
                '[' => parts.push(String::new()),
                ']' => {}
                _ => {
                    if let Some(last) = parts.last_mut() {
                        last.push(c);
                    } else {
                        parts.push(String::from(c));
                    }
                }
            }
        }
        QueryString(
            parts
                .into_iter()
                .map(|p| {
                    p.parse::<usize>()
                        .map(QueryStringPart::Index)
                        .unwrap_or_else(|_| QueryStringPart::Key(p))
                })
                .collect(),
        )
    }
}

impl From<String> for QueryString {
    fn from(value: String) -> Self {
        QueryString::from(value.as_str())
    }
}

impl Display for QueryString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(first) = self.0.first() {
            write!(f, "{}", first)?;
        }
        for part in self.0.iter().skip(1) {
            write!(f, "[{}]", part)?;
        }
        Ok(())
    }
}

impl QueryString {
    pub fn join(self, mut other: Self) -> Self {
        let mut parts = self.0;
        parts.append(&mut other.0);
        QueryString(parts)
    }

    pub fn add(mut self, part: QueryStringPart) -> Self {
        self.0.push(part);
        self
    }

    pub fn add_index(mut self, index: usize) -> Self {
        self.0.push(QueryStringPart::Index(index));
        self
    }

    pub fn add_key<K: Into<String>>(mut self, key: K) -> Self {
        self.0.push(QueryStringPart::Key(key.into()));
        self
    }
}

#[derive(Clone, Debug)]
pub struct FormDataSerialized(HashMap<QueryString, String>);

impl<F: Serialize> From<F> for FormDataSerialized {
    fn from(form_data: F) -> Self {
        let serialized = serde_qs::to_string(&form_data).expect("must be serializable");
        FormDataSerialized::from_query_str(&serialized)
    }
}

impl FormDataSerialized {
    pub fn from_query_str(string: &str) -> Self {
        let map = string
            .split("&")
            .into_iter()
            .map(|pair| {
                pair.split_once("=")
                    .map(|(k, v)| (QueryString::from(k), v.to_owned()))
                    .unwrap_or_else(|| (QueryString::from(pair), String::new()))
            })
            .collect();
        FormDataSerialized(map)
    }

    pub fn to_query_string(&self) -> String {
        self.0
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&")
    }

    pub fn exact(&self, key: &QueryString) -> Option<String> {
        self.0.get(&key).map(|s| s.to_owned())
    }

    pub fn level(&self, head: &QueryString) -> FormDataSerialized {
        let map = self
            .0
            .iter()
            .filter_map(|(k, v)| k.extends(head).map(|k| (k, v.to_owned())))
            .collect();
        FormDataSerialized(map)
    }

    pub fn len(&self) -> Option<usize> {
        self.0
            .keys()
            .map(|k| {
                k.0.first().and_then(|p| {
                    if let QueryStringPart::Index(i) = p {
                        Some(*i)
                    } else {
                        None
                    }
                })
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