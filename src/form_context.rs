use crate::{Data, FormData, QueryString, QueryStringPart};
use leptos::*;
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, ops::Deref, str::FromStr, sync::atomic::AtomicU64};

static VERSION: VersionProvider = VersionProvider::new();
pub type Version = u64;

struct VersionProvider(AtomicU64);

impl VersionProvider {
    const fn new() -> Self {
        Self(AtomicU64::new(0))
    }

    fn next(&self) -> Version {
        self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BaseGroupContext(GroupContext);

impl BaseGroupContext {
    pub(crate) fn new() -> Self {
        let group = GroupContext(RwSignal::new(GroupData {
            inputs: HashMap::new(),
            order: Vec::new(),
            qs: QueryString::default(),
            disabled: false,
            label: None,
        }));
        
        BaseGroupContext(group)
    }

    pub fn to_group_context(self) -> GroupContext {
        self.0
    }
}

impl Deref for BaseGroupContext {
    type Target = GroupContext;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputContext(RwSignal<InputData>);

#[derive(Debug, Clone)]
struct InputData {
    error: bool,
    disabled: bool,
    validate: Version,
    creation: Version,
    qs: QueryString,
    label: Option<TextProp>,
}

impl InputContext {
    pub fn new(bind: QueryStringPart) -> Self {
        let context = expect_context::<GroupContext>();
        Self::new_with_context(bind, context)
    }

    pub fn add_label(&self, label: TextProp) {
        self.0.update(|input| {
            input.label = Some(label);
        });
    }

    pub fn label(&self) -> Option<TextProp> {
        self.0.get_untracked().label
    }

    fn new_with_context(bind: QueryStringPart, context: GroupContext) -> Self {
        let version = VERSION.next();

        let input = InputContext(RwSignal::new(InputData {
            error: false,
            disabled: context.disabled().get_untracked(),
            qs: context.qs().add(bind),
            creation: version,
            validate: 0,
            label: None,
        }));

        context.register_input(bind, input);
        on_cleanup(move || {
            context.deregister(&bind);
        });    
        
        input
    }

    pub fn set_error(&self, has_error: bool) {
        self.0.update(|input| {
            input.error = has_error;
        });
    }

    pub fn error(&self) -> Signal<bool> {
        let self_signal = self.0;
        Memo::new(move |_| self_signal.get().error).into()
    }

    pub fn set_disabled(&self, disabled: bool) {
        self.0.update(|input| {
            input.disabled = disabled;
        });
    }

    pub fn disabled(&self) -> Signal<bool> {
        let self_signal = self.0;
        Memo::new(move |_| self_signal.get().disabled).into()
    }

    pub fn validate(&self) {
        self.0.update(|input| {
            input.validate = VERSION.next();
        });
    }

    pub fn validate_signal(&self) -> Signal<bool> {
        let self_signal = self.0;
        Memo::new(move |_| self_signal.get().validate > self_signal.get().creation).into()
    }

    pub fn qs(&self) -> QueryString {
        self.0.get_untracked().qs
    }

    pub fn raw_value(&self) -> Signal<String> {
        let qs = self.qs();
        Signal::derive(move || {
            expect_context::<FormData>()
                .get(qs)
                .get()
                .map(|data| data.into_input().unwrap().raw().to_owned())
                .unwrap_or_default()
     })
    }

    pub fn set_raw_value<T: ToString>(&self, value: T) {
        let qs = self.qs();
        expect_context::<FormData>().set(qs, Data::new_input(value.to_string()));
    }

    pub fn value<T: FromStr>(&self) -> Signal<Result<T, T::Err>> {
        let self_signal = self.raw_value();
        Signal::derive(move || {
            T::from_str(&self_signal.get())
        })
    }

    pub fn set_value<T: ToString>(&self, value: T) {
        self.set_raw_value(value);
    }

    pub fn get(&self, qs: QueryString) -> Signal<Option<Node>> {
        let self_copy = *self;
        Memo::new(move |_| {    
            if qs.is_empty() {
                Some(Node::Input(self_copy))
            } else {
                panic!("invalid query string");
            }
        }).into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupContext(RwSignal<GroupData>);

#[derive(Debug, Clone)]
struct GroupData {
    inputs: HashMap<QueryStringPart, Node>,
    order: Vec<QueryStringPart>,
    qs: QueryString,
    disabled: bool,
    label: Option<TextProp>,
}

impl GroupContext {
    pub fn new(bind: QueryStringPart) -> Self {
        let context = expect_context::<GroupContext>();
        Self::new_with_context(bind, context)
    }

    pub fn add_label(&self, label: TextProp) {
        self.0.update(|data| {
            data.label = Some(label);
        });
    }

    pub fn label(&self) -> Option<TextProp> {
        self.0.get_untracked().label
    }

    fn new_with_context(bind: QueryStringPart, context: GroupContext) -> Self {

        let group = GroupContext(RwSignal::new(GroupData {
            inputs: HashMap::new(),
            order: Vec::new(),
            qs: context.qs().add(bind),
            disabled: false,
            label: None,
        }));

        context.register_group(bind, group);
        on_cleanup(move || {
            context.deregister(&bind);
        });

        group
    }

    pub fn disabled(&self) -> Signal<bool> {
        let self_signal = self.0;
        Signal::derive(move || self_signal.get().disabled)
    }

    pub fn nodes(&self) -> Vec<Node> {
        self.0.get().order.iter().map(|qs| {
            self.0.get().inputs.get(qs).unwrap().clone()
        }).collect()
    }

    pub fn qs(&self) -> QueryString {
        self.0.get_untracked().qs
    }

    fn register_input(&self, qs: QueryStringPart, input: InputContext) {
        self.0.update(|data| {
            data.inputs.insert(qs, Node::Input(input));
            data.order.push(qs);
        });
    }

    fn register_group(&self, qs: QueryStringPart, group: GroupContext) {
        self.0.update(|data| {
            data.inputs.insert(qs, Node::Group(group));
            data.order.push(qs);
        });
    }

    pub fn deregister(&self, qs: &QueryStringPart) {
        self.0.update(|data| {
            data.inputs.remove(qs);
            data.order.retain(|k| k != qs);
        });
    }

    pub fn set_disabled(&self, disabled: bool) {
        logging::log!("set disabled: {}", disabled);
        self.0.update(|data| {
            data.disabled = disabled;
        });
        self.0.get_untracked().inputs.values().for_each(|node| {
            match node {
                Node::Input(input) => input.set_disabled(disabled),
                Node::Group(group) => group.set_disabled(disabled),
            }
        });
    }

    pub fn error(&self) -> Signal<bool> {
        let self_signal = self.0;
        Signal::derive(move || self_signal.get().inputs.values().any(|node| {
            match node {
                Node::Input(input) => input.error().get(),
                Node::Group(group) => group.error().get(),
            }
        }))
    }

    pub fn validate(&self) {
        self.0.get_untracked().inputs.values().for_each(|node| {
            match node {
                Node::Input(input) => input.validate(),
                Node::Group(group) => group.validate(),
            }
        });
    }

    pub fn len(&self) -> Signal<Option<usize>> {
        let qs = self.qs();
        Signal::derive(move || {
            expect_context::<FormData>().get(qs).get().map(|d| d.as_group().unwrap().len())
        })
    }

    pub fn raw_value(&self) -> Signal<Data> {
        let qs = self.qs();
        Signal::derive(move || {
            expect_context::<FormData>().get(qs).get().unwrap()
        })
    }

    pub fn set_raw_value(&self, data: Data) {
        let qs = self.qs();
        expect_context::<FormData>().set(qs, data);
    }


    pub fn value<T: DeserializeOwned + PartialEq>(&self) -> Signal<Option<T>> {
        let data = self.raw_value();

        Memo::new(move |_| {
            let deserialized = data.get().to::<T>();

            if cfg!(debug_assertions) {
                if let Err(err) = &deserialized {
                    logging::warn!("deserialization error, this may indicated that your form data is not matching your bindings: {:?}", err);
                }
            }
    
            Some(deserialized.ok()?)
        }).into()
    }

    pub fn set_value<T: Serialize>(&self, value: &T) {
        self.set_raw_value(Data::from(value));
    }

    pub fn get(&self, qs: QueryString) -> Signal<Option<Node>> {
        let self_copy = *self;
        Memo::new(move |_| {
            let mut qs = qs.iter();
            let (head, tail) = (qs.next(), qs.collect());
            if let Some(head) = head {
                self_copy.0.get().inputs.get(&head)?.get(tail).get()
            } else {
                Some(Node::Group(self_copy))
            }
        }).into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Node {
    Input(InputContext),
    Group(GroupContext),
}

impl Node {
    pub fn qs(&self) -> QueryString {
        match self {
            Node::Input(input) => input.qs(),
            Node::Group(group) => group.qs(),
        }
    }

    pub fn as_group(&self) -> Option<&GroupContext> {
        match self {
            Node::Input(_) => None,
            Node::Group(group) => Some(group),
        }
    }

    pub fn as_input(&self) -> Option<&InputContext> {
        match self {
            Node::Input(input) => Some(input),
            Node::Group(_) => None,
        }
    }

    pub fn into_group(self) -> GroupContext {
        match self {
            Node::Input(_) => panic!("expected group, got input"),
            Node::Group(group) => group,
        }
    }

    pub fn into_input(self) -> InputContext {
        match self {
            Node::Input(input) => input,
            Node::Group(_) => panic!("expected input, got group"),
        }
    }

    pub fn set_disabled(&self, disabled: bool) {
        match self {
            Node::Input(input) => input.set_disabled(disabled),
            Node::Group(group) => group.set_disabled(disabled),
        }
    }

    pub fn validate(&self) {
        match self {
            Node::Input(input) => input.validate(),
            Node::Group(group) => group.validate(),
        }
    }

    pub fn get(&self, qs: QueryString) -> Signal<Option<Node>> {
        match self {
            Node::Input(input) => input.get(qs),
            Node::Group(group) => group.get(qs),
        }
    }
}

pub fn node(qs: QueryString) -> Signal<Option<Node>> {
    let group = expect_context::<BaseGroupContext>();
    group.get(qs)
}

#[macro_export]
macro_rules! node {
    ( $key:ident $( [ $tail:tt ] )* ) => {
        node!(@part($crate::QueryString::default().add_key(stringify!($key))) $( [ $tail ] )*)
    };
    ( .. $( [ $tail:tt ] )* ) => {
        node!(@part(leptos::expect_context::<$crate::GroupContext>().qs()) $( [ $tail ] )*)
    };
    ( @part($part:expr) [ $index:literal ] $( [ $tail:tt ] )* ) => {
        node!(@part($part.add_index($index))$( [ $tail ] )*)
    };
    ( @part($part:expr) [ $key:ident ] $( [ $tail:tt ] )* ) => {
        node!(@part($part.add_key(stringify!($key))) $( [ $tail ] )*)
    };
    ( @part($part:expr) ) => {
        node($part)
    };
}

#[macro_export]
macro_rules! group {
    ( $($key:ident)? $( [ $tail:tt ] )* ) => {
        Signal::derive(move || Some(node!( $($key)? $( [ $tail ] )* ).get()?.into_group()))
    };
    ( .. $( [ $tail:tt ] )* ) => {
        Signal::derive(move || Some(node!( .. $( [ $tail ] )* ).get()?.into_group()))
    };
}

#[macro_export]
macro_rules! input {
    ( $($key:ident)? $( [ $tail:tt ] )* ) => {
        Signal::derive(move || Some(node!( $($key)? $( [ $tail ] )* ).get()?.into_input()))
    };
    ( .. $( [ $tail:tt ] )* ) => {
        Signal::derive(move || Some(node!( .. $( [ $tail ] )* ).get()?.into_input()))
    };
}

#[macro_export]
macro_rules! value {
    ( $($key:ident)? $( [ $tail:tt ] )* as $ty:ty ) => {
        Signal::derive(move || Some(input!( $($key)? $( [ $tail ] )* ).get()?.value::<$ty>().get().ok()?))
    };
    ( .. $( [ $tail:tt ] )* as $ty:ty ) => {
        Signal::derive(move || Some(input!( .. $( [ $tail ] )* ).get()?.value::<$ty>().get().ok()?))
    };
}

#[macro_export]
macro_rules! values {
    ( $($key:ident)? $( [ $tail:tt ] )* ) => {
        Signal::derive(move || Some(group!( $($key)? $( [ $tail ] )* ).get()?.raw_value().get()))
    };
    ( .. $( [ $tail:tt ] )* ) => {
        Signal::derive(move || Some(group!( .. $( [ $tail ] )* ).get()?.raw_value().get()))
    };
}

#[cfg(test)]
mod tests {
    use crate::qs;
    use super::*;

    #[test]
    fn test_node_macro() {
        let _ = leptos::create_runtime();

        let form_data = FormData::new();
        let group = BaseGroupContext::new();
        provide_context(form_data);
        provide_context(group);
        provide_context(group.to_group_context());

        let group = GroupContext::new(QueryStringPart::from("a"));
        let input = InputContext::new_with_context(QueryStringPart::from("b"), group);
        input.set_raw_value("test");

        let node = node!(a[b]);
        assert_eq!(node.get_untracked().unwrap().as_input().unwrap().qs(), qs!(a[b]));
    }

    #[test]
    fn test_input_macro() {
        let _ = leptos::create_runtime();

        let form_data = FormData::new();
        let group = BaseGroupContext::new();
        provide_context(form_data);
        provide_context(group);
        provide_context(group.to_group_context());

        let group = GroupContext::new(QueryStringPart::from("a"));
        let input = InputContext::new_with_context(QueryStringPart::from("b"), group);
        input.set_raw_value("test");

        let node = input!(a[b]);
        assert_eq!(node.get_untracked().unwrap().qs(), qs!(a[b]));
    }

    #[test]
    fn test_group_macro() {
        let _ = leptos::create_runtime();

        let form_data = FormData::new();
        let group = BaseGroupContext::new();
        provide_context(form_data);
        provide_context(group);
        provide_context(group.to_group_context());

        let group = GroupContext::new(QueryStringPart::from("a"));
        let input = InputContext::new_with_context(QueryStringPart::from("b"), group);
        input.set_raw_value("test");

        let node = group!(a);
        assert_eq!(node.get_untracked().unwrap().nodes().len(), 1);
    }

    #[test]
    fn test_value_macro() {
        let _ = leptos::create_runtime();

        let form_data = FormData::new();
        let group = BaseGroupContext::new();
        provide_context(form_data);
        provide_context(group);
        provide_context(group.to_group_context());

        let group = GroupContext::new(QueryStringPart::from("a"));
        let input = InputContext::new_with_context(QueryStringPart::from("b"), group);
        input.set_raw_value("test");

        let value = value!(a[b] as String);
        assert_eq!(value.get_untracked().unwrap(), "test");
    }

    #[test]
    fn test_input() {
        let _ = leptos::create_runtime();

        let form_data = FormData::new();
        let group = BaseGroupContext::new();
        provide_context(form_data);
        provide_context(group.to_group_context());

        let input = InputContext::new(QueryStringPart::from("a"));
        input.set_raw_value("test");

        assert_eq!(form_data.get(qs!(a)).get_untracked().unwrap().as_input().unwrap().raw(), "test");
    }

    #[test]
    fn test_input_group() {
        let _ = leptos::create_runtime();

        let form_data = FormData::new();
        let group = BaseGroupContext::new();
        provide_context(form_data);
        provide_context(group.to_group_context());

        let group = GroupContext::new(QueryStringPart::from("a"));
        let input = InputContext::new_with_context(QueryStringPart::from("b"), group);
        input.set_raw_value("test");

        assert_eq!(form_data.get(qs!(a[b])).get_untracked().unwrap().as_input().unwrap().raw(), "test");
    }

    #[test]
    fn test_input_group_modify() {
        let _ = leptos::create_runtime();

        let form_data = FormData::new();
        let group = BaseGroupContext::new();
        provide_context(form_data);
        provide_context(group.to_group_context());

        let group = GroupContext::new(QueryStringPart::from("a"));
        let input = InputContext::new_with_context(QueryStringPart::from("b"), group);
        input.set_raw_value("test");

        assert_eq!(form_data.get(qs!(a[b])).get_untracked().unwrap().as_input().unwrap().raw(), "test");

        input.set_raw_value("test2");
        assert_eq!(form_data.get(qs!(a[b])).get_untracked().unwrap().as_input().unwrap().raw(), "test2");
    }

    #[test]
    fn test_input_group_validate() {
        let _ = leptos::create_runtime();

        let form_data = FormData::new();
        let group = BaseGroupContext::new();
        provide_context(form_data);
        provide_context(group.to_group_context());

        let group = GroupContext::new(QueryStringPart::from("a"));
        let input1 = InputContext::new_with_context(QueryStringPart::from("b"), group);
        let input2 = InputContext::new_with_context(QueryStringPart::from("c"), group);
        group.validate();
        let input3 = InputContext::new_with_context(QueryStringPart::from("d"), group);

        assert_eq!(input1.validate_signal().get_untracked(), true);
        assert_eq!(input2.validate_signal().get_untracked(), true);
        assert_eq!(input3.validate_signal().get_untracked(), false);
    }

    #[test]
    fn test_input_group_disable() {
        let _ = leptos::create_runtime();

        let form_data = FormData::new();
        let group = BaseGroupContext::new();
        provide_context(form_data);
        provide_context(group.to_group_context());

        let group = GroupContext::new(QueryStringPart::from("a"));
        let input1 = InputContext::new_with_context(QueryStringPart::from("b"), group);
        let input2 = InputContext::new_with_context(QueryStringPart::from("c"), group);
        group.set_disabled(true);
        let input3 = InputContext::new_with_context(QueryStringPart::from("d"), group);

        assert_eq!(input1.disabled().get_untracked(), true);
        assert_eq!(input2.disabled().get_untracked(), true);
        assert_eq!(input3.disabled().get_untracked(), true);
    }

    #[test]
    fn test_input_group_error() {
        let _ = leptos::create_runtime();

        let form_data = FormData::new();
        let group = BaseGroupContext::new();
        provide_context(form_data);
        provide_context(group.to_group_context());

        let group = GroupContext::new(QueryStringPart::from("a"));
        let _input1 = InputContext::new_with_context(QueryStringPart::from("b"), group);
        assert_eq!(group.error().get_untracked(), false);
        
        let input2 = InputContext::new_with_context(QueryStringPart::from("c"), group);
        input2.set_error(true);
        let _input3 = InputContext::new_with_context(QueryStringPart::from("d"), group);

        assert_eq!(group.error().get_untracked(), true);
    }
}