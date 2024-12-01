use std::{fmt::Display, hash::Hash, str::FromStr};

use crate::{use_translation, QueryString, FieldWiring};
use leptos::*;
use strum::{IntoEnumIterator, ParseError};

/// A component that renders radio buttons from an enum.
#[component]
pub fn Radio<T>(
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
 
    view! {
        <div
            class="field radio"
            class:error=move || error.get().is_some()
            class:ok=move || error.get().is_none()
        >
        { move || {
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
                    <fieldset>
                        <legend>{label.clone()}</legend>
                        <For
                            each={move || T::iter()}
                            key={|item| *item}
                            children={move |item| {
                                let input_elem = html::input()
                                    .attr("type", "radio")
                                    .attr("id", format!("{}({})", qs.to_string(), item.into()))
                                    .attr("name", qs.to_string())
                                    .attr("checked", move || value.get() == Ok(item))
                                    .attr("value", move || item.into())
                                    .on(ev::input, move |ev| {
                                        set_raw_value.call(event_target_value(&ev));
                                    });

                                view! {
                                    <label for=format!("{}({})", qs.to_string(), item.into())>
                                        {input_elem}
                                        <span class="custom-radio"></span>
                                        <span class="custom-radio-label">{use_translation(item)}</span>
                                    </label>
                                }
                            }}
                        />
                        {move || {
                            if let Some(error) = error.get() {
                                view! { <span class="error-message">{error}</span> }
                                    .into_view()
                            } else {
                                View::default()
                            }
                        }}
                    </fieldset>
                }.into_view()
            }
        }}    
        </div>
    }
}
