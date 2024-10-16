use leptos::*;

use crate::{Group, QueryString};

use super::IconButton;

#[component]
pub fn Repeatable<F, IV>(#[prop(into)] bind: QueryString, item: F) -> impl IntoView
where
    F: Fn(usize) -> IV + 'static,
    IV: IntoView,
{
    let (_qs, form_data) = bind.form_context();
    let (size, set_size) = create_signal(form_data.len().unwrap_or(0));
    let item = store_value(item);

    view! {
        <Group bind=bind>
            <div class="repeatable">
                <For
                    each= move || (0..size.get())
                    key= |i| *i
                    children = move |i| {
                        view! {
                            <Group bind=QueryString::default().add_index(i)>
                                {item.with_value(|item| item(i))}
                            </Group>
                        }
                    }
                />
                <IconButton on:click = move |_| set_size.update(|i| *i -= 1) label="Remove" icon="remove" disabled=Signal::derive(move || size.get() == 0) />
                <IconButton on:click = move |_| set_size.update(|i| *i += 1) label="Add" icon="add" />
            </div>
        </Group>
    }
}
