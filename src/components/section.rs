use leptos::*;

use crate::{Group, QueryStringPart};

#[component]
pub fn Section(
    #[prop(into)] title: TextProp,
    #[prop(into, optional)] description: Option<TextProp>,
    #[prop(optional)] children: Option<Children>,
    /// An optional binding that creates a new group.
    #[prop(into, optional)] bind: Option<QueryStringPart>,
) -> impl IntoView {

    let label_clone = title.clone();

    let section = move || view! {
        <section class="section">
            <h3 class="section-title">{title}</h3>
            {if let Some(description) = description { view! {
                <p class="section-description">{description}</p>
            }.into_view() } else { View::default() }}
            {if let Some(children) = children { children().into_view() } else { View::default() }}
        </section>
    };
    
    if let Some(bind) = bind {
       view! {
            <Group bind=bind label=label_clone>
                {section()}
            </Group>
       }.into_view()
    } else {
        section().into_view()
    }
}
