use leptos::*;

use crate::{FormData, QueryString};

/// A component that binds all of its contents to a part of the form data.
#[component]
pub fn Group(
    /// The query string that binds the group to the form data.
    #[prop(into)] bind: QueryString,
    /// The children of the group.
    children: Children
) -> impl IntoView {
    let qs = bind.context();
    let form_data = FormData::with_context(&qs);

    view! {
        <Provider value=qs>
            <Provider value=form_data>{children()}</Provider>
        </Provider>
    }
}
