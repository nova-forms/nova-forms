use std::{fmt::Debug, hash::Hash, str::FromStr};

use leptos::*;

use crate::Icon;

#[component]
pub fn SelectButton<V, F, G>(
    #[prop(into)] label: TextProp,
    #[prop(into, optional)] icon: Option<String>,
    #[prop(into)] id: String,
    #[prop(into)] values: Vec<(V, TextProp)>,
    value: G,
    on_change: F,
) -> impl IntoView
where
    V: FromStr + ToString + Eq + Hash + Clone + 'static,
    <V as FromStr>::Err: Debug,
    F: Fn(V) + Copy + 'static,
    G: Fn() -> V + Copy + 'static,
{
    let options = view! {
        <For
            each=move || values.clone()
            key=|(value, _)| value.clone()
            children=move |(v, d)| {
                view! { <option value=v.to_string()>{d}</option> }
            }
        />
    };

    view! {
        <label class="overlay icon-select button" for=id.clone()>
            {
                if let Some(icon) = icon {
                    view! { <Icon label=label icon=icon /> }.into_view()
                } else {
                    view! { <span>{label}</span> }.into_view()
                }
            }
            <select
                id=id
                on:change=move |ev| {
                    let value = event_target_value(&ev);
                    let value = V::from_str(&value).unwrap();
                    on_change(value)
                }
                prop:value=move || value().to_string()
                name="language"
            >
                {options}
            </select>
        </label>
    }
}
