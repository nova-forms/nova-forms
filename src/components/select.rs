use std::{fmt::Display, hash::Hash, str::FromStr};

use crate::{use_translation, QueryString, FieldWiring};
use html::Select;
use leptos::*;
use strum::{IntoEnumIterator, ParseError};

/// A component that renders a select field from an enum.
#[component]
pub fn Select<T>(
    /// The label of the input field.
    #[prop(into)] label: TextProp,
    /// The query string that binds the input field to the form data.
    #[prop(into)] bind: QueryString,
    /// The initial value of the input field.
    #[prop(optional, into)] value: MaybeProp<T>,
    /// A write signal that is updated with the parsed value of the input field.
    #[prop(optional, into)] change: Option<Callback<Result<T, T::Err>, ()>>,
    /// Set a custom error message for the input field.
    #[prop(optional, into)] error: MaybeProp<TextProp>,
) -> impl IntoView
where
    T: IntoEnumIterator + FromStr<Err = ParseError> + Into<&'static str> + Clone + Copy + Default + Eq + Hash + Display + 'static
{    
    let FieldWiring {
        qs,
        value,
        error,
        set_raw_value,
        render_mode,
        ..
    } = FieldWiring::<T>::wire(bind, value, change, error);

    // Get value on load from the input field.
    let node_ref = NodeRef::<Select>::new();
    node_ref.on_load(move |element| {
        let element: &web_sys::HtmlSelectElement = &*element;
        let value = element.value();
        if !value.is_empty() {
            set_raw_value.call(value);
        }
    });
 
    let select_elem = view! {
        <select _ref=node_ref id=qs.to_string() name=qs.to_string() on:input=move |ev| {
            set_raw_value.call(event_target_value(&ev));
        }>
            <For
                each={move || T::iter()}
                key={|item| *item}
                children={move |item| {
                    let option_elem = html::option()
                        .attr("id", format!("{}({})", qs.to_string(), item.into()))
                        .attr("selected", move || value.get() == Ok(item))
                        .attr("value", move || item.into())
                        .child(use_translation(item));
                    
                    view! {
                        {option_elem}
                    }
                }}
            />
        </select>
    };

    view! {
        <div
            class="field select"
            class:error=move || error.get().is_some()
            class:ok=move || error.get().is_none()
        >
            {move || {
                if render_mode.get() {
                    if let Ok(value) = value.get() {
                        view!{
                            <span class="label">{label.clone()}</span>
                            <span class="value">{use_translation(value)}</span>
                        }.into_view()
                    } else {
                        view!{
                            <span class="label">{label.clone()}</span>
                            <span class="value"></span>
                        }.into_view()
                    }
                } else {
                    view! {
                        <label for=qs.to_string()>{label.clone()}</label>
                        {select_elem.clone()}
                        {move || {
                            if let Some(error) = error.get() {
                                view! { <span class="error-message">{error}</span> }
                                    .into_view()
                            } else {
                                View::default()
                            }
                        }}
                    }.into_view()
                }
            }}
        </div>
    }
}
