use leptos::*;

#[component]
pub fn Icon(
    #[prop(into)] label: String,
    #[prop(into)] icon: String,
) -> impl IntoView
{
    view! {
        <span class="material-symbols-rounded" aria-label=label>{icon}</span>
    }
}
