use itertools::Itertools;
use leptos::*;

/// A component renders multiple columns.
#[component]
pub fn Cols(
    /// The number of columns. The default is two.
    #[prop(optional, default=2)] cols: usize,
    children: Children,
) -> impl IntoView
where
{    
    view! {
        <div class="cols" style=format!("grid-template-columns: {};", (0..cols).map(|_i| format!("1fr")).join(" "))>
            {children()}
        </div>
    }
}

/// A component creates a colspan over multiple columns.
#[component]
pub fn Colspan(
    /// The number of columns to span over. The default is two.
    #[prop(optional, default=2)] cols: usize,
    children: Children,
) -> impl IntoView
where
{    
    view! {
        <div class="colspan" style=format!("grid-column: span {};", cols)>
            {children()}
        </div>
    }
}
