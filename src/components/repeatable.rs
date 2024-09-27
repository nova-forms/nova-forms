use leptos::*;

use crate::{FormDataSerialized, Group, QueryString};

#[component]
pub fn Repeatable<F, IV>(#[prop(into)] bind: QueryString, item: F) -> impl IntoView
where
    F: Fn(usize) -> IV + 'static,
    IV: IntoView,
{
    /* 
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
                <button on:click = move |_| set_size.update(|i| *i += 1)>"Add child"</button>
                <Show when=move || { size.get() > 0 } >
                    <button on:click = move |_| set_size.update(|i| *i -= 1)>"Remove child"</button>
                </Show>
            </div>
        </Group>
    }
    */

    let form_data = expect_context::<FormDataSerialized>();
    let curr_form_data = form_data.level(&bind);
    let prefix_qs = expect_context::<QueryString>();
    let curr_qs = prefix_qs.join(bind.clone());
    let (size, set_size) = create_signal(curr_form_data.len().unwrap_or(0));
    let item = store_value(item);

    view! {
        <div class="repeatable">
            <For
                each= move || (0..size.get())
                key= |i| *i
                children = move |i| {
                    let curr_qs = QueryString::from(format!("{curr_qs}[{i}]"));
                    let curr_form_data = curr_form_data.level(&QueryString::from(format!("{i}")));

                    view! {
                        <Provider value=curr_qs>
                            <Provider value=curr_form_data>
                                {item.with_value(|item| item(i))}
                            </Provider>
                        </Provider>
                    }
                }
            />
            <button on:click = move |_| set_size.update(|i| *i += 1)>"Add child"</button>
            <Show when=move || { size.get() > 0 } >
                <button on:click = move |_| set_size.update(|i| *i -= 1)>"Remove child"</button>
            </Show>
        </div>
    }
}
