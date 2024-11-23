use std::{fmt::Display, str::FromStr};

use crate::{use_translation, FormData, NovaFormContext, QueryString};
use html::{ElementDescriptor, Input, Select};
use leptos::*;
pub trait WiringTarget: ElementDescriptor + Clone + 'static {
    fn get_value(element: HtmlElement<Self>) -> String where Self: Sized;
}

impl WiringTarget for Input {
    fn get_value(element: HtmlElement<Self>) -> String {
        let element: &web_sys::HtmlInputElement = &*element;
        element.value()
    }
}

impl WiringTarget for Select {
    fn get_value(element: HtmlElement<Self>) -> String {
        let element: &web_sys::HtmlSelectElement = &*element;
        element.value()
    }
}

pub struct FieldWiring<T, W>
where
    T: FromStr + ToString + Clone + Default + 'static,
    T::Err: Clone + Display,
    W: WiringTarget,
{
    pub qs: QueryString,
    pub node_ref: NodeRef<W>,
    pub value: Signal<Result<T, T::Err>>,
    pub raw_value: Signal<String>,
    pub set_value: Callback<T, ()>,
    pub set_raw_value: Callback<String, ()>,
    pub error: Signal<Option<TextProp>>,
    pub render_mode: Signal<bool>,
}

impl<T, W> FieldWiring<T, W>
where
    T: FromStr + ToString + Clone + Default + Display + 'static,
    T::Err: Clone + Display,
    W: WiringTarget,
{
    pub fn wire(
        label: TextProp,
        bind: QueryString,
        value: MaybeProp<T>,
        change: Option<Callback<Result<T, T::Err>, ()>>,
        error: MaybeProp<TextProp>,
    ) -> Self {
        let nova_form_context = expect_context::<NovaFormContext>();
        let qs = bind.context();
        let form_data = FormData::with_context(&qs);
        let form_value = form_data.value();
        let raw_form_value = form_data.raw_value();
        let (input_typed, set_input_typed) = create_signal(false);
    
        // Set debug value.
        if cfg!(debug_assertions) {
            form_data.set_value(T::default());
        }
    
        // Get value on load from the input field.
        let node_ref = NodeRef::new();
        let form_data_clone = form_data.clone();
        node_ref.on_load(move |node| {
            let value = W::get_value(node);
            if !value.is_empty() {
                logging::log!("set value on load {}", value);
                form_data_clone.set_raw_value(value);
                set_input_typed.set(true);
            }
        });
    
        // Register and deregister the input field.
        let validation_trigger = nova_form_context.register_input(qs.clone(), label.clone());
        on_cleanup(move || {
            nova_form_context.deregister_input(qs);
        });
    
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
                logging::log!("set value from setter {}", value);
                form_data_clone.set_value(value);
            }
        });
    
        let form_data_clone = form_data.clone();
        let set_raw_value = Callback::new(move |value| {
            logging::log!("set raw value with callback {}", value);
            form_data_clone.set_raw_value(value);
            set_input_typed.set(true);
        });

        let form_data_clone = form_data.clone();
        let set_value = Callback::new(move |value| {
            logging::log!("set value with callback {}", value);
            form_data_clone.set_value(value);
            set_input_typed.set(true);
        });

        let show_error = Signal::derive(move || {
            validation_trigger.get() || input_typed.get()
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

        create_effect(move |_| {
            logging::log!("raw value is {}", raw_form_value.get());
        });
    
        FieldWiring {
            qs,
            node_ref,
            value: form_value,
            raw_value: raw_form_value,
            set_raw_value,
            set_value,
            error,
            render_mode
        }
    }
}
