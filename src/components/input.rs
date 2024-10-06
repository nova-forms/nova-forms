use leptos::*;

use crate::{Datatype, QueryString};

#[component]
pub fn Input<T>(
    #[prop(into)] label: TextProp,
    #[prop(into)] bind: QueryString,
    #[prop(optional, into)] placeholder: Option<T>,
    #[prop(optional, into)] value: Option<Signal<Option<T>>>,
) -> impl IntoView
where
    T: Datatype,
{
    let (qs, form_value) = bind.form_value::<T>();
    let (error, set_error) = create_signal(Ok(T::default()));

    let input_elem = T::attributes()
        .into_iter()
        .fold(html::input(), |el, (name, value)| el.attr(name, value))
        .attr("id", qs.to_string())
        .attr("name", qs.to_string())
        .attr("value", move || value.map(|value| value.get()).flatten().or_else(|| form_value.clone().ok()).map(|value| value.to_string()))
        .attr("placeholder", placeholder.as_ref().map(T::to_string))
        .on(ev::input, move |ev| {
            set_error.set(T::validate(event_target_value(&ev)));
        });

    view! {
        <div class={move || { if error.get().is_err() { "field error" } else { "field" }}} >
            <label for=qs.to_string()>{label}</label>
            /*<input id=qs.to_string() type="text" name=qs.to_string() value=format!("{}", value.unwrap_or_default()) placeholder=placeholder.as_ref().map(T::to_string).unwrap_or_default() on:input=move |ev| {
                set_error.set(T::validate(event_target_value(&ev)));
            }/>*/
            { input_elem }
            /*<Show when=move || error.get().is_err()>
                <span class="error-message">{error.get().err().expect("must be an error")}</span>
            </Show>*/
        </div>
    }
}