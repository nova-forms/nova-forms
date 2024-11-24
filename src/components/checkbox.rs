use crate::{Datatype, QueryString, FieldWiring};
use leptos::*;

/// A component that renders a checkbox.
#[component]
pub fn Checkbox<T>(
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
    T: Datatype<Inner = bool>
{    
    let FieldWiring {
        qs,
        raw_value,
        error,
        set_raw_value,
        ..
    } = FieldWiring::<T>::wire(label.clone(), bind, value, change, error);

    // Get value on load from the input field.
    let node_ref = NodeRef::new();
    node_ref.on_load(move |element| {
        let element: &web_sys::HtmlInputElement = &*element;
        let value = element.checked();
        set_raw_value.call(value.to_string());
    });

    let input_elem = html::input()
        .attr("type", "checkbox")
        .attr("id", qs.to_string())
        .attr("name", qs.to_string())
        .attr("checked", move || raw_value.get())
        .attr("value", true.to_string())
        .node_ref(node_ref)
        .on(ev::input, move |ev| {
            set_raw_value.call(event_target_checked(&ev).to_string());
        });


    view! {
        <div
            class="field checkbox"
            class:error=move || error.get().is_some()
            class:ok=move || error.get().is_none()
        >
            <label for=qs.to_string()>
                {input_elem}
                <span class="custom-checkbox"></span>
                <span class="custom-checkbox-label">{label}</span>
            </label>
            {move || {
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
