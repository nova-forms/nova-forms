use std::fmt::{Debug, Display};

use leptos::*;
use ustr::Ustr;

/// A part of a query string.
/// Either an index for arrays or a key to access a value.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum QueryStringPart {
    Index(usize),
    Key(Ustr),
}

impl Display for QueryStringPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryStringPart::Index(i) => write!(f, "{}", i),
            QueryStringPart::Key(k) => write!(f, "{}", k),
        }
    }
}

impl From<&str> for QueryStringPart {
    fn from(value: &str) -> Self {
        QueryStringPart::Key(Ustr::from(value))
    }
}

impl From<String> for QueryStringPart {
    fn from(value: String) -> Self {
        QueryStringPart::from(value.as_str())
    }
}

impl From<usize> for QueryStringPart {
    fn from(value: usize) -> Self {
        QueryStringPart::Index(value)
    }
}

/// Used to bind a form input element to a form data element.
/// Note that `QueryString` supports a maximal depth of 16.
/// Creating query strings consisting of more than 16 parts will panic.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub struct QueryString {
    parts: [Option<QueryStringPart>; 16],
    len: usize,
}

impl FromIterator<QueryStringPart> for QueryString {
    fn from_iter<T: IntoIterator<Item = QueryStringPart>>(iter: T) -> Self {
        let mut qs = QueryString::default();
        let mut len = 0;
        for (i, part) in iter.into_iter().enumerate() {
            qs.parts[i] = Some(part);
            len += 1;
        }
        qs.len = len;
        qs
    }
}

impl<'a> FromIterator<&'a QueryStringPart> for QueryString {
    fn from_iter<T: IntoIterator<Item = &'a QueryStringPart>>(iter: T) -> Self {
        let mut qs = QueryString::default();
        let mut len = 0;
        for (i, part) in iter.into_iter().enumerate() {
            qs.parts[i] = Some(*part);
            len += 1;
        }
        qs.len = len;
        qs
    }
}

impl QueryString {
    pub fn iter(&self) -> impl Iterator<Item = &QueryStringPart> {
        self.parts.iter().flatten().fuse()
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Checks whether the current query string extends the other query string.
    pub fn extends(&self, other: &Self) -> Option<QueryString> {
        debug_assert_eq!(self.parts.iter().filter(|p| p.is_some()).count(), self.len);
        debug_assert_eq!(other.parts.iter().filter(|p| p.is_some()).count(), other.len);

        if self.len() < other.len() {
            return None;
        }

        if !self.iter().zip(other.iter()).all(|(s, o)| s == o) {
            return None;
        }

        Some(self.iter().skip(other.len()).collect())
    }

    /// Joins two `QueryString`s.
    pub fn join(self, other: Self) -> Self {
        self.iter().chain(other.iter()).collect()
    }

    pub fn add(mut self, part: QueryStringPart) -> Self {
        self.parts[self.len] = Some(part);
        self.len += 1;
        debug_assert_eq!(self.parts.iter().filter(|p| p.is_some()).count(), self.len);
        self
    }

    pub fn add_index(self, index: usize) -> Self {
        self.add(QueryStringPart::Index(index))
    }

    pub fn add_key<K: AsRef<str>>(self, key: K) -> Self {
        self.add(QueryStringPart::Key(Ustr::from(key.as_ref())))
    }

    pub fn first(&self) -> Option<QueryStringPart> {
        self.parts[0]
    }

    pub fn remove(mut self) -> Self {
        self.parts[self.len - 1] = None;
        self.len -= 1;
        debug_assert_eq!(self.parts.iter().filter(|p| p.is_some()).count(), self.len);
        self
    }

    pub fn remove_first(mut self) -> Self {
        for i in 1..self.len {
            self.parts[i - 1] = self.parts[i];
        }
        self.parts[self.len - 1] = None;
        self.len -= 1;
        debug_assert_eq!(self.parts.iter().filter(|p| p.is_some()).count(), self.len);
        self
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

        parts
            .into_iter()
            .map(|p| {
                p.parse::<usize>()
                    .map(QueryStringPart::Index)
                    .unwrap_or_else(|_| QueryStringPart::Key(Ustr::from(&p)))
            })
            .collect()
    }
}

impl From<String> for QueryString {
    fn from(value: String) -> Self {
        QueryString::from(value.as_str())
    }
}

impl Display for QueryString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
        }
        for part in iter {
            write!(f, "[{}]", part)?;
        }
        Ok(())
    }
}

/// Creates a `QueryString`.
#[macro_export]
macro_rules! qs {
    ( $key:ident $($t:tt)* ) => {
        qs!(@part($crate::QueryString::default().add_key(stringify!($key))) $($t)*)
    };
    ( .. $($t:tt)* ) => {
        qs!(@part(leptos::expect_context::<$crate::GroupContext>().qs()) $($t)*)
    };
    () => {
        $crate::QueryString::default()
    };
    ( @part($part:expr) [!] $($t:tt)* ) => {
        qs!(@part($part.remove()) $($t)*)
    };
    ( @part($part:expr) [ $index:literal ] $($t:tt)* ) => {
        qs!(@part($part.add_index($index)) $($t)*)
    };
    ( @part($part:expr) [ $key:ident ] $($t:tt)* ) => {
        qs!(@part($part.add_key(stringify!($key))) $($t)*)
    };
    ( @part($part:expr) ) => {
        $part
    };
}

#[cfg(test)]
mod tests {
    use crate::{BaseGroupContext, GroupContext};

    use super::*;

    #[test]
    fn test_extends() {
        assert_eq!(QueryString::from("form_data[a][b]").extends(&QueryString::from("form_data[a]")), Some(QueryString::from("b")));
    }

    #[test]
    fn test_join() {
        assert_eq!(QueryString::from("a").join(QueryString::from("b")), QueryString::from("a[b]"));
    }

    #[test]
    fn test_add() {
        assert_eq!(QueryString::from("a").add_key("b"), QueryString::from("a[b]"));
        assert_eq!(QueryString::from("a").add_index(0), QueryString::from("a[0]"));
    }

    #[test]
    fn test_qs_context() {
        let _ = leptos::create_runtime();
        provide_context(BaseGroupContext::new().to_group_context());
        provide_context(GroupContext::new(QueryStringPart::from("a")));
        expect_context::<GroupContext>();
        let qs = qs!(..[b][c]);
        assert_eq!(qs, QueryString::from("a[b][c]"));
    }

    #[test]
    fn test_qs_remove() {
        let qs = qs!(a[b][d][!][c]);
        assert_eq!(qs, QueryString::from("a[b][c]"));
    }
    
    
    #[test]
    fn test_qs_macro() {
        let qs = qs!(a[b][c]);
        assert_eq!(qs, QueryString::from("a[b][c]"));
        let qs = qs!(a[0][c]);
        assert_eq!(qs, QueryString::from("a[0][c]"));
        let qs = qs!(a);
        assert_eq!(qs, QueryString::from("a"));
        let qs = qs!();
        assert_eq!(qs, QueryString::default());
    }

    #[test]
    fn test_first() {
        let qs = qs!(a[b][c]);
        assert_eq!(qs.first(), Some(QueryStringPart::Key(Ustr::from("a"))));
    }

    #[test]
    fn test_remove() {
        let qs = qs!(a[b][c]);
        assert_eq!(qs.remove(), QueryString::from("a[b]"));
    }

    #[test]
    fn test_remove_first() {
        let qs = qs!(a[b][c]);
        assert_eq!(qs.remove_first(), QueryString::from("b[c]"));
    }
}
