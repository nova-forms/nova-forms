use std::{fmt::Display, str::FromStr};

use crate::{use_translation, FormContext, InputContext, QueryString, QueryStringPart};
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
    pub error: Signal<Option<Oco<'static, str>>>,
    pub render_mode: Signal<bool>,
    pub disabled: Signal<bool>,
}

impl<T> FieldWiring<T>
where
    T: FromStr + ToString + Clone + Default + 'static,
    T::Err: Clone + Display
{
    pub fn wire(
        bind: QueryStringPart,
        external_value: MaybeProp<T>,
        change: Option<Callback<Result<T, T::Err>, ()>>,
        error: MaybeProp<TextProp>,
        label: TextProp,
    ) -> Self {
        let input = InputContext::new(bind);  
        input.add_label(label);
        let qs = input.qs();
        let validate_signal = input.validate_signal();
        let nova_form_context = expect_context::<FormContext>();
        let form_value = input.value();
        let raw_form_value = input.raw_value();
        let (show_error, set_show_error) = create_signal(false);

        // Set debug value.
        if cfg!(debug_assertions) {
            input.set_value(T::default());
        }

        // Call change callback if value changed.
        if let Some(change) = change {
            create_effect(move |_| {
                logging::log!("call change callback");
                change.call(form_value.get());
            });
        }
    
        // Update value
        create_effect(move |_| {
            if let Some(value) = external_value.get() {
                logging::log!("update value from external");
                input.set_value(value);
            }
        });
    
        let set_raw_value = Callback::new(move |value: String| {
            logging::log!("set_raw_value {}: {}", qs, value);
            input.set_raw_value(value);
            set_show_error.set(true);
        });

        let set_value = Callback::new(move |value: T| {
            logging::log!("set_value {}: {}", qs, value.to_string());
            input.set_value(value);
            set_show_error.set(true);
        });

        create_effect(move |_| {
            logging::log!("validate_signal {}", qs);
            if validate_signal.get() {
                set_show_error.set(true);
            }
        });

        create_effect(move |_| {
            match form_value.get() {
                Ok(value) => {
                    logging::log!("form_value changed {}: {}", qs, value.to_string());
                }
                Err(err) => {
                    logging::log!("form_value changed {}: {}", qs, err);
                }
            }
        });

        let error_message = Memo::new(move |_| {
            logging::log!("error_message {}", qs);
            if let Some(error) = error.get() {
                Some(error.get())
            } else
            if show_error.get() {
                form_value.get().err().map(|err| use_translation(err).get())
            } else {
                None
            }
        }).into();

        let disabled = input.disabled();

        let render_mode = Signal::derive(move || nova_form_context.is_render_mode());
    
        FieldWiring {
            qs,
            value: form_value,
            raw_value: raw_form_value,
            set_raw_value,
            set_value,
            error: error_message,
            render_mode,
            disabled,
        }
    }
}
