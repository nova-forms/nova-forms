use leptos::*;

use crate::Button;

use super::DialogKind;

/// A modal dialog.
#[component]
pub fn Modal(
    #[prop(into)] id: String,
    /// The kind of dialog to display.
    kind: DialogKind,
    /// Whether the dialog is open.
    #[prop(into)] open: Signal<bool>,
    /// The callback to close the dialog.
    #[prop(into, optional)] close: Option<Callback<(), ()>>,
    /// The title of the dialog.
    #[prop(into)] title: TextProp,
    /// The message of the dialog.
    #[prop(into)] msg: TextProp,
) -> impl IntoView {
    
    let id_clone = id.clone();
    create_effect(move |_| {
        if open.get() {
            js_sys::eval(format!(r#"
                document.getElementById('{id}').showModal();
                document.getElementById('{id}').addEventListener('cancel', (event) => {{
                    event.preventDefault();
                }});
            "#, id=id_clone).as_str())
                .expect("Failed to show modal");
        } else {
            js_sys::eval(format!("document.getElementById('{}').close()", id_clone).as_str())
                .expect("Failed to close modal");
        }
    });

    view! {
        <Show when=move || open.get()>
            //<Body attr:inert=move || open.get() />
            <div class="modal-background"></div>
        </Show>
        <dialog
            id=id
            class=format!("modal {}", kind.class())
            tabindex="-1"
        >
            <div class="modal-header">{
                let title = title.clone();
                move || title.clone()
            }</div>
            <div class="modal-main">{
                let msg = msg.clone();
                move || msg.clone()
            }</div>
            {
                if let Some(close) = close {
                    view! {
                        <div class="modal-footer">
                            <Button icon="close" label="Close" on:click=move |_ev| close.call(()) />
                        </div>
                    }.into_view()
                } else {
                    View::default()
                }
            }
        </dialog>
    }
}
