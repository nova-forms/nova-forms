use leptos::*;

use crate::QueryString;

#[component]
pub fn Group(#[prop(into)] bind: QueryString, children: Children) -> impl IntoView {
    let (qs, form_data) = bind.form_context();

    view! {
        <Provider value=qs>
            <Provider value=form_data>
                {children()}
            </Provider>
        </Provider>
    }
}
