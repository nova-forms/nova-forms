use std::{fmt::Display, str::FromStr};

use crate::{use_translation, FormContext, InputContext, QueryString};
use leptos::*;

/// Used to wire an input field to the form context.
/// For example, refer to the input field components in this library.
pub struct FieldWiring<T>
where
    T: FromStr + ToString + Clone + Default + 'static,
    T::Err: Clone + Display,
{
    pub qs: QueryString,
    pub value: Signal<Result<T, T::Err>>,
    pub raw_value: Signal<String>,
    pub set_value: Callback<T, ()>,
    pub set_raw_value: Callback<String, ()>,
    pub error: Signal<Option<TextProp>>,
    pub render_mode: Signal<bool>,
}

impl<T> FieldWiring<T>
where
    T: FromStr + ToString + Clone + Default + Display + 'static,
    T::Err: Clone + Display
{
    pub fn wire(
        bind: QueryString,
        value: MaybeProp<T>,
        change: Option<Callback<Result<T, T::Err>, ()>>,
        error: MaybeProp<TextProp>,
    ) -> Self {
        let input = InputContext::new(bind);
        let qs = input.qs();
        let validate_signal = input.validate_signal();
        let nova_form_context = expect_context::<FormContext>();
        let form_value = input.value();
        let raw_form_value = input.raw_value();
        let (input_typed, set_input_typed) = create_signal(false);

        // Set debug value.
        if cfg!(debug_assertions) {
            input.set_value(T::default());
        }

        // Call change callback if value changed.
        if let Some(change) = change {
            create_effect(move |_| {
                change.call(form_value.get());
            });
        }
    
        // Update value
        create_effect(move |_| {
            if let Some(value) = value.get() {
                input.set_value(value);
            }
        });
    
        let set_raw_value = Callback::new(move |value| {
            input.set_raw_value(value);
            set_input_typed.set(true);
        });

        let set_value = Callback::new(move |value| {
            input.set_value(value);
            set_input_typed.set(true);
        });

        let show_error = Signal::derive(move || {
            validate_signal.get() || input_typed.get()
        });

        let error = Signal::derive(move || {
            if let Some(error) = error.get() {
                Some(error)
            } else
            if show_error.get() {
                form_value.get().err().map(|err| use_translation(err))
            } else {
                None
            }
        });

        let render_mode = Signal::derive(move || nova_form_context.is_render_mode());
    
        FieldWiring {
            qs,
            value: form_value,
            raw_value: raw_form_value,
            set_raw_value,
            set_value,
            error,
            render_mode
        }
    }
}
