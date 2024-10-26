use crate::{Datatype, NovaFormContext, QueryString, TranslationProvider};
use leptos::*;

/// A component that renders an input field.
/// It takes a datatype as a type parameter and automatically handles parsing and validation.
#[component]
pub fn Checkbox<T>(
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
    T: Datatype<Inner = bool> + Into<bool>,
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
                    .into())
        } else {
            input_value.get()
                .unwrap_or_else(|| value.get()
                    .or_else(|| form_value.clone().ok())
                    .map(|v| v.into())
                    .unwrap_or_default())
        }
    });

    let nova_form_context = expect_context::<NovaFormContext>();
    let validation_trigger = nova_form_context.register_input(qs.clone(), label.clone());

    let show_error = Signal::derive(move || {
        input_value.get().is_some() || validation_trigger.get()
    });

    let parsed_value = Signal::derive(move || T::validate(raw_value.get()));

    let qs_clone = qs.clone();
    on_cleanup(move || {
        nova_form_context.deregister_input(qs_clone);
    });

    let qs_clone = qs.clone();
    create_effect(move |_| {
        let qs = qs_clone.clone();
        nova_form_context.set_error(&qs, parsed_value.get().is_err());
    });

    let input_elem = T::attributes()
        .into_iter()
        .fold(html::input(), |el, (name, value)| el.attr(name, value))
        .attr("type", "checkbox")
        .attr("id", qs.to_string())
        .attr("name", qs.to_string())
        .attr("checked", move || raw_value.get())
        .attr("value", move || raw_value.get().to_string())
        .on(ev::input, move |ev| {
            set_input_value.set(Some(event_target_checked(&ev)));
        });

    let translate_errors = use_context::<TranslationProvider<<T as Datatype>::Error>>();

    view! {
        <div
            class="field"
            class:error=move || parsed_value.get().is_err() && show_error.get()
            class:ok=move || parsed_value.get().is_ok() && show_error.get()
        >
            <label for=qs.to_string()>
                {input_elem}
                <span class="custom-checkbox"></span>
                <span class="custom-checkbox-label">{label}</span>
            </label>
            {move || {
                if let (Err(err), Some(translate_errors), true) = (
                    parsed_value.get(),
                    translate_errors.as_ref(),
                    show_error.get(),
                ) {
                    view! { <span class="error-message">{translate_errors.clone().t(err)}</span> }
                        .into_view()
                } else {
                    View::default()
                }
            }}
        </div>
    }
}
