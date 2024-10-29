use leptos::*;

use crate::IconButton;

use super::DialogKind;

#[component]
pub fn Modal(
    kind: DialogKind,
    #[prop(into)] open: Signal<bool>,
    #[prop(into, optional)] close: Option<Callback<(), ()>>,
    #[prop(into)] title: TextProp,
    #[prop(into)] msg: TextProp,
) -> impl IntoView {
    view! {
        <Show when=move || open.get()>
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
                    <div class="modal-footer">
                        {
                            if let Some(close) = close {
                                view! {
                                    <IconButton icon="close" label="Close" on:click=move |_ev| close.call(()) />
                                }.into_view()
                            } else {
                                View::default()
                            }
                        }
                    </div>
                </dialog>
            </div>
        </Show>
    }
}
