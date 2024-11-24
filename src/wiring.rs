use std::{fmt::Display, str::FromStr};

use crate::{use_translation, FormContext, FormData, InputContext, QueryString};
use leptos::*;
use ustr::Ustr;

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
        label: TextProp,
        bind: QueryString,
        value: MaybeProp<T>,
        change: Option<Callback<Result<T, T::Err>, ()>>,
        error: MaybeProp<TextProp>,
    ) -> Self {
        let label_str = Ustr::from(label.clone().get().as_str());
        let input = InputContext::new(bind, label);
        let qs = input.qs();
        let validate_signal = input.validate_signal();
        let nova_form_context = expect_context::<FormContext>();
        let form_data = FormData::with_context(&qs);
        let form_value = form_data.value();
        let raw_form_value = form_data.raw_value();
        let (input_typed, set_input_typed) = create_signal(false);

        // Set debug value.
        if cfg!(debug_assertions) {
            form_data.set_value(T::default());
        }

        // Call change callback if value changed.
        if let Some(change) = change {
            create_effect(move |_| {
                change.call(form_value.get());
            });
        }
    
        // Update value
        let form_data_clone = form_data.clone();
        create_effect(move |_| {
            if let Some(value) = value.get() {
                form_data_clone.set_value(value);
            }
        });
    
        let form_data_clone = form_data.clone();
        let set_raw_value = Callback::new(move |value| {
            form_data_clone.set_raw_value(value);
            set_input_typed.set(true);
        });

        let form_data_clone = form_data.clone();
        let set_value = Callback::new(move |value| {
            form_data_clone.set_value(value);
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
