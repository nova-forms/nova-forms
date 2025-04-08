use leptos::*;

use crate::{Data, GroupContext, QueryStringPart};

/// A component that binds all of its contents to a part of the form data.
#[component]
pub fn Group(
    /// An optional label for the group.
    #[prop(optional, into)] label: Option<TextProp>,
    /// The query string that binds the group to the form data.
    #[prop(into)] bind: QueryStringPart,
    /// The value of the group.
    #[prop(optional, into)] values: MaybeProp<Data>,
     /// The value of the group.
     #[prop(optional, into)] disabled: MaybeProp<bool>,
    /// The children of the group.
    children: Children
) -> impl IntoView
{
    let group = GroupContext::new(bind);
    if let Some(label) = label {
        group.add_label(label);
    }

    create_effect(move |_| {
        if let Some(values) = values.get() {
            group.set_raw_value(values);
        }
    });

    create_effect(move |_| {
        if let Some(disabled) = disabled.get() {
            logging::log!("group set disabled: {}", disabled);
            group.set_disabled(disabled);
        }
    });

    view! {
        <Provider value=group>
            {children()}
        </Provider>
    }
}
