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
        raw_value,
        error,
        set_raw_value,
        render_mode,
        ..
    } = FieldWiring::<T, Select>::wire(label.clone(), bind, value, change, error);
 
    view! {
        <div
            class="field select"
            class:error=move || error.get().is_some()
            class:ok=move || error.get().is_none()
        >
            <label for=qs.to_string()>{label}</label>
            {move || {
                let qs = qs.clone();

                if render_mode.get() {
                    let text_elem = html::input()
                        .attr("type", "text")
                        .attr("readonly", true)
                        .attr("id", qs.to_string())
                        .attr("name", qs.to_string())
                        .attr("value", move || raw_value.get());

                    text_elem.into_view()
                } else {
                    view! {
                        <select id=qs.to_string() name=qs.to_string() on:input=move |ev| {
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
                    }.into_view()
                }
            }}
            {move || {
                if let Some(error) = error.get() {
                    view! { <span class="error-message">{error}</span> }
                        .into_view()
                } else
                if let Some(error) = error.get() {
                    view! { <span class="error-message">{error}</span> }
                        .into_view()
                } else {
                    View::default()
                }
            }}
        </div>
    }
}
