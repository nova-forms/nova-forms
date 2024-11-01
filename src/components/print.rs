use leptos::*;

/// Only renders the children when printing to PDF.
#[component]
pub fn PrintOnly(
    children: Children,
) -> impl IntoView
where
{    
    view! {
        <div class="print-only" style=format!("")>
            {children()}
        </div>
    }
}

/// Only renders the children when printing to PDF.
#[component]
pub fn ScreenOnly(
    children: Children,
) -> impl IntoView
where
{    
    view! {
        <div class="screen-only" style=format!("")>
            {children()}
        </div>
    }
}