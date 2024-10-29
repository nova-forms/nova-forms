use leptos::*;

#[derive(Copy, Clone, Debug)]
pub enum ModalKind {
    Success,
    Info,
    Warn,
    Error,
}

#[component]
pub fn Modal(
    kind: ModalKind,
    #[prop(into)] title: TextProp,
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] close: Callback<(), ()>,
    #[prop(into)] msg: TextProp,
) -> impl IntoView {
    view! {
        <Show when=move || open.get()>
            <div class="modal">
                <dialog
                    open=open.get()
                    aria-modal="true"
                    class=match kind {
                        ModalKind::Success => "success",
                        ModalKind::Info => "info",
                        ModalKind::Warn => "warn",
                        ModalKind::Error => "error",
                    }
                >
                    <div class="modal-header">{title.clone()}</div>
                    <div class="modal-main">{msg.clone()}</div>
                    <div class="modal-footer">
                        <button type="button" on:click=move |_ev| close.call(())>
                            "Confirm"
                        </button>
                    </div>
                </dialog>
            </div>
        </Show>
    }
}
