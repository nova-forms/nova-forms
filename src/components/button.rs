use leptos::*;

use crate::Icon;

/// A button that only contains an icon.
#[component]
pub fn Button(
    #[prop(optional, into)] button_type: Option<String>,
    #[prop(into)] label: TextProp,
    #[prop(into, optional)] icon: Option<String>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] form: Option<String>,
    #[prop(optional, into)] disabled: Option<MaybeSignal<bool>>,
) -> impl IntoView {
    view! {
        <button
            class="overlay icon-button button"
            disabled=move || disabled.map(|s| s.get()).unwrap_or_default()
            type=button_type.unwrap_or("button".to_owned())
            form=form
            id=id.unwrap_or_default()
        >
        {
            if let Some(icon) = icon {
                view! { <Icon label=label icon=icon /> }.into_view()
            } else {
                view! { <span>{label}</span> }.into_view()
            }
        }
        </button>
    }
}

/// A button that only contains an icon.
#[component]
pub fn ButtonGroup(
    children: Children
) -> impl IntoView {
    view! {
        <div class="button-group">
            {children()}
        </div>
    }
}
