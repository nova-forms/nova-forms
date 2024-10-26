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
    #[prop(into)] close: Callback<(), ()>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="modal">
            <dialog
                open
                aria-modal="true"
                class=match kind {
                    ModalKind::Success => "success",
                    ModalKind::Info => "info",
                    ModalKind::Warn => "warn",
                    ModalKind::Error => "error",
                }
            >
                <div class="modal-header">{title}</div>
                <div class="modal-main">{children()}</div>
                <div class="modal-footer">
                    <button type="button" on:click=move |_ev| close.call(())>
                        "Confirm"
                    </button>
                </div>
            </dialog>
        </div>
    }
}
