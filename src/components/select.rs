use std::{fmt::{Debug, Display}, hash::Hash, str::FromStr};

use leptos::*;

#[component]
pub fn Select<V, D, F, G>(
    #[prop(into)] values: Vec<(V, D)>,
    value: G,
    on_change: F,
) -> impl IntoView
where
    V: FromStr + ToString + Eq + Hash + Clone + 'static,
    <V as FromStr>::Err: Debug,
    D: Display + Clone + 'static,
    F: Fn(V) + Copy + 'static,
    G: Fn() -> V + Copy + 'static
{

    let options = view! {
        <For
            each=move || values.clone()
            key=|(value, _)| value.clone()
            children = move |(v, d)| {
                view! {
                    <option value=v.to_string()>{format!("{d}")}</option>
                }
            }
        />
    };
    
    view! {
        <select
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
    }
}