use super::PageId;
use crate::{Datatype, QueryString, FieldWiring};
use leptos::*;

mod context {
    use super::PageId;
    use crate::QueryString;
    use leptos::*;
    use std::{borrow::Cow, collections::HashMap, str::FromStr};

    #[derive(Debug, Clone)]
    pub(crate) struct InputData {
        pub(crate) page_id: Option<PageId>,
        #[allow(unused)]
        pub(crate) label: TextProp,
        pub(crate) has_error: bool,
        #[allow(unused)]
        pub(crate) version: u64,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub(crate) struct InputId(Cow<'static, str>);

    impl ToString for InputId {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    impl FromStr for InputId {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(InputId(Cow::Owned(s.to_owned())))
        }
    }

    #[derive(Debug, Clone)]
    pub(crate) struct InputsContext {
        inputs: HashMap<QueryString, InputData>,
    }

    impl InputsContext {
        pub fn new() -> Self {
            Self {
                inputs: HashMap::new(),
            }
        }

        pub fn register(&mut self, qs: QueryString, data: InputData) {
            self.inputs.insert(qs, data);
        }

        pub fn deregister(&mut self, qs: &QueryString) {
            self.inputs.remove(qs);
        }

        pub fn set_error(&mut self, qs: &QueryString, has_error: bool) {
            self.inputs.get_mut(qs).expect("cannot set error").has_error = has_error;
        }

        pub fn has_errors(&self) -> Option<&InputData> {
            self.inputs.values().find(|data| data.has_error)
        }
    }
}

pub(crate) use context::*;


/// A component that renders an input field.
/// It takes a datatype as a type parameter and automatically handles parsing and validation.
#[component]
pub fn Input<T>(
    /// The label of the input field.
    #[prop(into)] label: TextProp,
    /// The query string that binds the input field to the form data.
    #[prop(into)] bind: QueryString,
    /// The placeholder text of the input field.
    #[prop(optional, into)] placeholder: Option<T>,
    /// The initial value of the input field.
    #[prop(optional, into)] value: MaybeProp<T>,
    /// A write signal that is updated with the parsed value of the input field.
    #[prop(optional, into)] change: Option<Callback<Result<T, T::Error>, ()>>,
    /// Set a custom error message for the input field.
    #[prop(optional, into)] error: MaybeProp<TextProp>,
) -> impl IntoView
where
    T: Datatype,
{    
    let FieldWiring {
        qs,
        node_ref,
        raw_value,
        error,
        set_raw_value,
        render_mode,
        ..
    } = FieldWiring::wire(label.clone(), bind, value, change, error);

    let text_elem = T::attributes()
        .into_iter()
        .filter(|(name, _)| *name != "type")
        .fold(html::input(), |el, (name, value)| el.attr(name, value))
        .attr("type", "text")
        .attr("readonly", true)
        .attr("id", qs.to_string())
        .attr("name", qs.to_string())
        .prop("value", move || raw_value.get());

    let input_elem = T::attributes()
        .into_iter()
        .fold(html::input(), |el, (name, value)| el.attr(name, value))
        .attr("id", qs.to_string())
        .attr("name", qs.to_string())
        .attr("placeholder", placeholder.as_ref().map(T::to_string))
        .prop("value", move || raw_value.get())
        .node_ref(node_ref)
        .on(ev::input, move |ev| {
            set_raw_value.call(event_target_value(&ev));
        });

    view! {
        <div
            class="field"
            class:error=move || error.get().is_some()
            class:ok=move || error.get().is_none()
        >
            <label for=qs.to_string()>{label}</label>
            {move || {
                if render_mode.get() {
                    text_elem.clone()
                } else {
                    input_elem.clone()
                }
            }}
            {move || {
                if let Some(error) = error.get() {
                    view! { <span class="error-message">{error}</span> }
                        .into_view()
                } else
                if let Some(error) = error.get(){
                    view! { <span class="error-message">{error}</span> }.into_view()
                } else {
                    View::default()
                }
            }}
        </div>
    }
}
