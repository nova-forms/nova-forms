use super::PageId;
use crate::{use_translation, Datatype, NovaFormContext, QueryString};
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
    #[prop(optional, into)] debug_value: Option<T>,
    /// The initial value of the input field.
    #[prop(optional, into)] value: MaybeProp<T>,
) -> impl IntoView
where
    T: Datatype,
{    
    let (qs, form_value) = bind.form_value::<T>();

    let (input_value, set_input_value) = create_signal(None);

    let raw_value = Signal::derive(move || {
        if cfg!(debug_assertions) {
            input_value.get()
                .unwrap_or_else(|| value.get()
                    .or_else(|| form_value.clone().ok())
                    .or_else(|| debug_value.clone())
                    .unwrap_or_else(|| T::default_debug_value())
                    .to_string())
        } else {
            input_value.get()
                .unwrap_or_else(|| value.get()
                    .or_else(|| form_value.clone().ok())
                    .map(|v| v.to_string())
                    .unwrap_or_default())
        }
    });

    let nova_form_context = expect_context::<NovaFormContext>();
    let validation_trigger = nova_form_context.register_input(qs.clone(), label.clone());

    let show_error = Signal::derive(move || {
        input_value.get().is_some() || validation_trigger.get()
    });

    let parsed_value = Signal::derive(move || T::from_str(&raw_value.get()));

    let qs_clone = qs.clone();
    on_cleanup(move || {
        nova_form_context.deregister_input(qs_clone);
    });

    let qs_clone = qs.clone();
    create_effect(move |_| {
        let qs = qs_clone.clone();
        nova_form_context.set_error(&qs, parsed_value.get().is_err());
    });

    view! {
        <div
            class="field"
            class:error=move || parsed_value.get().is_err() && show_error.get()
            class:ok=move || parsed_value.get().is_ok() && show_error.get()
        >
            <label for=qs.to_string()>{label}</label>
            {move || {

                if nova_form_context.is_render_mode() {
                    let text_elem = T::attributes()
                        .into_iter()
                        .filter(|(name, _)| *name != "type")
                        .fold(html::input(), |el, (name, value)| el.attr(name, value))
                        .attr("type", "text")
                        .attr("readonly", true)
                        .attr("id", qs.to_string())
                        .attr("name", qs.to_string())
                        .attr("value", move || raw_value.get());

                    text_elem
                } else {
                    let input_elem = T::attributes()
                        .into_iter()
                        .fold(html::input(), |el, (name, value)| el.attr(name, value))
                        .attr("id", qs.to_string())
                        .attr("name", qs.to_string())
                        .attr("value", move || raw_value.get())
                        .attr("placeholder", placeholder.as_ref().map(T::to_string))
                        .on(ev::input, move |ev| {
                            set_input_value.set(Some(event_target_value(&ev)));
                        });

                    input_elem
                }
            }}
            {move || {
                if let (Err(err), true) = (
                    parsed_value.get(),
                    show_error.get(),
                ) {
                    view! { <span class="error-message">{use_translation(err)}</span> }
                        .into_view()
                } else {
                    View::default()
                }
            }}
        </div>
    }
}
