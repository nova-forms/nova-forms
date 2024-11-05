use leptos::*;
use leptos_meta::Body;

use crate::Button;

use super::DialogKind;

/// A modal dialog.
#[component]
pub fn Modal(
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
    view! {
        <Show when=move || open.get()>
            <Body attr:inert=move || open.get() />
            <div class="modal">
                <dialog
                    open=open.get()
                    aria-modal="true"
                    class=kind.class()
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
            </div>
        </Show>
    }
}
