use std::{fmt::Display, hash::Hash, str::FromStr};

use crate::{use_translation, NovaFormContext, QueryString};
use leptos::*;
use strum::{IntoEnumIterator, ParseError};

/// A component that renders a select field from an enum.
#[component]
pub fn Select<T>(
    /// The label of the input field.
    #[prop(into)] label: TextProp,
    /// The query string that binds the input field to the form data.
    #[prop(into)] bind: QueryString,
    /// The placeholder text of the input field.
    #[prop(optional, into)] debug_value: Option<T>,
    /// The initial value of the input field.
    #[prop(optional, into)] value: MaybeProp<T>,
) -> impl IntoView
where
    T: IntoEnumIterator + FromStr<Err = ParseError> + Into<&'static str> + Clone + Copy + Default + Eq + Hash + Display + 'static
{    
    let (qs, form_value) = bind.form_value::<T>();

    let (input_value, set_input_value) = create_signal(None);

    let raw_value = Signal::derive(move || {
        if cfg!(debug_assertions) {
            input_value.get()
                .unwrap_or_else(|| value.get()
                    .or_else(|| form_value.clone().ok())
                    .or_else(|| debug_value.clone())
                    .unwrap_or_else(|| T::default())
                    .into()
                    .to_string())
        } else {
            input_value.get()
                .unwrap_or_else(|| value.get()
                    .or_else(|| form_value.clone().ok())
                    .map(|v| v.into().to_string())
                    .unwrap_or_default())
        }
    });

    let nova_form_context = expect_context::<NovaFormContext>();
    let validation_trigger = nova_form_context.register_input(qs.clone(), label.clone());

    let show_error = Signal::derive(move || {
        input_value.get().is_some() || validation_trigger.get()
    });

    let parsed_value = Signal::derive(move || T::from_str(raw_value.get().as_str()));

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
            class="field select"
            class:error=move || parsed_value.get().is_err() && show_error.get()
            class:ok=move || parsed_value.get().is_ok() && show_error.get()
        >
            <label for=qs.to_string()>{label}</label>
            {move || {
                let qs = qs.clone();

                if nova_form_context.is_render_mode() {
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
                            set_input_value.set(Some(event_target_value(&ev)));
                        }>
                            <For
                                each={move || T::iter()}
                                key={|item| *item}
                                children={move |item| {
                                    let option_elem = html::option()
                                        .attr("id", format!("{}({})", qs.to_string(), item.into()))
                                        .attr("selected", move || parsed_value.get() == Ok(item))
                                        .attr("value", move || item.into())
                                        .on(ev::input, move |ev| {
                                            set_input_value.set(Some(event_target_value(&ev)));
                                        })
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
