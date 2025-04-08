use std::collections::{BTreeMap, HashMap};

use leptos::*;
use serde::{de::DeserializeOwned, Serialize};

use crate::{QueryString, QueryStringPart};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FormData(RwSignal<Data>);

impl FormData {
    pub fn new() -> Self {
        Self(RwSignal::new(Data::new_group()))
    }

    pub fn from_data(data: Data) -> Self {
        Self(RwSignal::new(data))
    }

    pub fn get(&self, qs: QueryString) -> Signal<Option<Data>> {
        let self_singal = self.0;
        Memo::new(move |_| {
            self_singal.get().get(qs)
        }).into()
    }

    pub fn set(&self, qs: QueryString, value: Data) {
        self.0.update(|data| {
            data.set(qs, value);
        });
    }

    pub fn from_urlencoded(data: &str) -> Self {
        Self::from_data(Data::from_urlencoded(data))
    }

    pub fn from<T: Serialize>(value: &T) -> Self {
        Self::from_data(Data::from(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Data {
    Group(GroupData),
    Input(InputData),
}

impl Data {
    pub fn new_group() -> Self {
        Data::Group(GroupData::new())
    }

    pub fn new_input(value: String) -> Self {
        Data::Input(InputData::new(value))
    }

    fn from_vec(data: Vec<(QueryString, String)>) -> Self {
        let mut parts = HashMap::new();

        for (k, v) in data {
            if let Some(head) = k.first() {
                let vec: &mut Vec<(QueryString, String)> = parts
                    .entry(head)
                    .or_default();

                vec.push((k.remove_first(), v));
            } else {
                return Data::Input(InputData(v));
            }
        }

        let group = parts.into_iter()
            .map(|(head, v)| {
                (head, Data::from_vec(v))
            })
            .collect();

            Data::Group(GroupData(group))
    }

    fn to_vec(&self) -> Vec<(QueryString, String)> {
        fn visit(data: &Data, qs: QueryString, acc: &mut Vec<(QueryString, String)>) {
            match data {
                Data::Group(group) => {
                    for (head, data) in &group.0 {
                        visit(data, qs.add(*head), acc);
                    }
                }
                Data::Input(value) => {
                    acc.push((qs, value.0.clone()));
                }
            }
        }

        let mut acc = Vec::new();
        visit(self, QueryString::default(), &mut acc);
        acc
    }

    pub fn from_urlencoded(data: &str) -> Self {
        let vec = data.split('&').into_iter()
            .map(|part| part.split_once('=').map(|(k, v)| {
                (QueryString::from(k), v.to_owned())
            }).unwrap_or((QueryString::from(part), String::new())))
            .collect::<Vec<_>>();

        Self::from_vec(vec)
    }

    pub fn to_urlencoded(&self) -> String {
        self.to_vec().into_iter()
            .map(|(k, v)| {
                format!("{}={}", k.to_string(), v)
            })
            .collect::<Vec<_>>()
            .join("&")
    }

    pub fn to<T: DeserializeOwned>(&self) -> Result<T, serde_qs::Error> {
        serde_qs::from_str(&self.to_urlencoded())
    }

    pub fn from<T: Serialize>(value: &T) -> Self {
        let data = serde_qs::to_string(value).expect("failed to serialize form data");
        Data::from_urlencoded(&data)
    }

    pub fn get(&self, qs: QueryString) -> Option<Data> {
        if let Some(first) = qs.first() {
            if let Data::Group(group) = self {
                group.0.get(&first)?.get(qs.remove_first())
            } else {
                None
            }
        } else {
            Some(self.clone())
        }
    }

    pub fn set(&mut self, qs: QueryString, value: Data) {
        if let Some(first) = qs.first() {
            if let Data::Group(group) = self {
                let data = group.0.entry(first).or_insert_with(|| Data::new_group());
                data.set(qs.remove_first(), value);
            }
        } else {
            *self = value;
        }
    }

    pub fn len(&self) -> Option<usize> {
        match self {
            Data::Group(group) => Some(group.len()),
            _ => None,
        }
    }

    pub fn as_input(&self) -> Option<&InputData> {
        match self {
            Data::Input(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_group(&self) -> Option<&GroupData> {
        match self {
            Data::Group(group) => Some(group),
            _ => None,
        }
    }

    pub fn into_input(self) -> Option<InputData> {
        match self {
            Data::Input(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_group(self) -> Option<GroupData> {
        match self {
            Data::Group(group) => Some(group),
            _ => None,
        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupData(BTreeMap<QueryStringPart, Data>);

impl GroupData {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputData(String);

impl InputData {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn raw(&self) -> &str {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::qs;
    use super::*;

    #[test]
    fn test_from_urlencoded() {
        let _ = leptos::create_runtime();

        let form_data = FormData::from_urlencoded("a=1&b=2&c=3");
        assert_eq!(form_data.get(qs!(a)).get_untracked().unwrap().as_input().unwrap().raw(), "1");
        assert_eq!(form_data.get(qs!(b)).get_untracked().unwrap().as_input().unwrap().raw(), "2");
        assert_eq!(form_data.get(qs!(c)).get_untracked().unwrap().as_input().unwrap().raw(), "3");
    }

    #[test]
    fn test_to_urlencoded() {
        let _ = leptos::create_runtime();

        let form_data = FormData::from_urlencoded("a=1&b=2&c=3");
        assert_eq!(form_data.get(qs!(a)).get_untracked().unwrap().as_input().unwrap().raw(), "1");
        assert_eq!(form_data.get(qs!(b)).get_untracked().unwrap().as_input().unwrap().raw(), "2");
        assert_eq!(form_data.get(qs!(c)).get_untracked().unwrap().as_input().unwrap().raw(), "3");
        assert_eq!(form_data.get(qs!()).get_untracked().unwrap().to_urlencoded(), "a=1&b=2&c=3");
    }

    
    #[test]
    fn test_set_value() {
        let _ = leptos::create_runtime();

        let form_data = FormData::from_urlencoded("a=1&b=2&c=3");
        form_data.set(qs!(b), Data::new_input("7".to_owned()));
        assert_eq!(form_data.get(qs!(a)).get_untracked().unwrap().as_input().unwrap().raw(), "1");
        assert_eq!(form_data.get(qs!(b)).get_untracked().unwrap().as_input().unwrap().raw(), "7");
        assert_eq!(form_data.get(qs!(c)).get_untracked().unwrap().as_input().unwrap().raw(), "3");

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

        let form_data = FormData::from_urlencoded("a=1&b=2&c=3");
        assert_eq!(form_data.get(qs!()).get_untracked().unwrap().to::<Test>().unwrap(), Test { a: 1, b: 2, c: 3 });
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

        let form_data = FormData::from_urlencoded("a=1&b=2&c=3");
        form_data.set(qs!(), Data::from(&Test { a: 4, b: 5, c: 6 }));
        assert_eq!(form_data.get(qs!()).get_untracked().unwrap().to::<Test>().unwrap(), Test { a: 4, b: 5, c: 6 });
    }

    #[test]
    fn test_len() {
        let _ = leptos::create_runtime();

        let form_data = FormData::from_urlencoded("a[0]=1&a[3]=21&a[2]=23&a[1]=3");
        assert_eq!(form_data.get(qs!(a)).get_untracked().unwrap().as_group().unwrap().len(), 4);
    }
}