use leptos::*;

use crate::Icon;

#[component]
pub fn IconButton(
    #[prop(optional, into)] button_type: Option<String>,
    #[prop(into)] label: TextProp,
    #[prop(into)] icon: String,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] form: Option<String>,
) -> impl IntoView
{
    view! {
        <button class="icon-button" type=button_type.unwrap_or("button".to_owned()) form=form id=id.unwrap_or_default() >
            <Icon label=label icon=icon />
        </button>
    }
}
