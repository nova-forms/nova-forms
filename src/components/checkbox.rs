use crate::{Datatype, QueryStringPart, FieldWiring};
use leptos::*;

/// A component that renders a checkbox.
#[component]
pub fn Checkbox<T>(
    /// The label of the input field.
    #[prop(into)] label: TextProp,
    /// The query string that binds the input field to the form data.
    #[prop(into)] bind: QueryStringPart,
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
        value,
        error,
        set_raw_value,
        render_mode,
        ..
    } = FieldWiring::<T>::wire(bind, value, change, error, label.clone());
    
    let input_elem = html::input()
        .attr("type", "checkbox")
        .attr("id", qs.to_string())
        .attr("name", qs.to_string())
        .attr("checked", move || value.get().unwrap_or_default().into())
        .attr("value", true.to_string())
        .on(ev::input, move |ev| {
            set_raw_value.call(event_target_checked(&ev).to_string());
        });


    view! {
        <div
            class="field checkbox"
            class:error=move || error.get().is_some()
            class:ok=move || error.get().is_none()
        >
        {move || {
            if render_mode.get() {
                // TODO
                view! {
                    <label for=qs.to_string()>
                        {input_elem.clone()}
                        <span class="custom-checkbox"></span>
                        <span class="custom-checkbox-label">{label.clone()}</span>
                    </label>
                    {move || {
                        if let Some(error) = error.get() {
                            view! { <span class="error-message">{error}</span> }
                                .into_view()
                        } else {
                            View::default()
                        }
                    }}
                }.into_view()
            } else {
                view! {
                    <label for=qs.to_string()>
                        {input_elem.clone()}
                        <span class="custom-checkbox"></span>
                        <span class="custom-checkbox-label">{label.clone()}</span>
                    </label>
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
