use leptos::*;

use crate::{ButtonGroup, GroupContext, QueryString};

use super::{Button, Group};

/// Creates a repeatable group of items.
#[component]
pub fn Repeatable<F, IV>(
    /// The query string that binds the repeatable group to a `Vec`.
    #[prop(into)] bind: QueryString,
    /// The item that is repeated.
    item: F
) -> impl IntoView
where
    F: Fn(usize) -> IV + 'static,
    IV: IntoView,
{
    let item = store_value(item);

    view! {
        <Group bind=bind>
            {
                let group = expect_context::<GroupContext>();
                let (size, set_size) = create_signal(group.form_data().len().unwrap_or(0));

                view! {
                    <div class="repeatable">
                        <For
                            each=move || (0..size.get())
                            key=|i| *i
                            children=move |i| {
                                view! {
                                    <Group bind=QueryString::default()
                                        .add_index(i)>{item.with_value(|item| item(i))}</Group>
                                }
                            }
                        />
                        <ButtonGroup>
                            <Button
                                on:click=move |_| set_size.update(|i| *i -= 1)
                                label="Remove"
                                icon="remove"
                                disabled=Signal::derive(move || size.get() == 0)
                            />
                            <Button on:click=move |_| set_size.update(|i| *i += 1) label="Add" icon="add" />                    
                        </ButtonGroup>
                    </div>
                }
            }
        </Group>
        
    }
}
