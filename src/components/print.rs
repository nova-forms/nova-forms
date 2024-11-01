use itertools::Itertools;
use leptos::*;

/// Only renders the children when printing to PDF.
#[component]
pub fn Print(
    children: Children,
) -> impl IntoView
where
{    
    view! {
        <div style=format!("")>
            {children()}
        </div>
    }
}