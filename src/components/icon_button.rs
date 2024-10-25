use leptos::*;

use crate::Icon;

/// A button that only contains an icon.
#[component]
pub fn IconButton(
    #[prop(optional, into)] button_type: Option<String>,
    #[prop(into)] label: TextProp,
    #[prop(into)] icon: String,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] form: Option<String>,
    #[prop(optional, into)] disabled: Option<MaybeSignal<bool>>,
) -> impl IntoView {
    view! {
        <button class="ui icon-button" disabled=move || disabled.map(|s| s.get()).unwrap_or_default() type=button_type.unwrap_or("button".to_owned()) form=form id=id.unwrap_or_default() >
            <Icon label=label icon=icon />
        </button>
    }
}
