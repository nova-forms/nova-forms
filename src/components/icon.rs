use leptos::*;

// Renders an icon.
// All available icons can be found at the [material symbols and iconswebsite from google](https://fonts.google.com/icons).
#[component]
pub fn Icon(#[prop(into)] label: TextProp, #[prop(into)] icon: String) -> impl IntoView {
    view! {
        <span class="material-symbols-rounded" aria-label=label>
            {icon}
        </span>
    }
}