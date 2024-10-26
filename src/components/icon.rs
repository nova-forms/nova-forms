use leptos::*;

#[component]
pub fn Icon(#[prop(into)] label: TextProp, #[prop(into)] icon: String) -> impl IntoView {
    view! {
        <span class="material-symbols-rounded" aria-label=label>
            {icon}
        </span>
    }
}
