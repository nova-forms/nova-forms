use leptos::*;

use crate::IconButton;

#[derive(Copy, Clone, Debug)]
pub enum DialogKind {
    Success,
    Info,
    Warn,
    Error,
}

impl DialogKind {
    pub fn class(&self) -> &'static str {
        match self {
            DialogKind::Success => "success",
            DialogKind::Info => "info",
            DialogKind::Warn => "warn",
            DialogKind::Error => "error",
        }
    }
}

/// A dialog component.
#[component]
pub fn Dialog(
    /// The kind of dialog to display.
    kind: DialogKind,
    /// Whether the dialog is open.
    #[prop(into, optional, default=true.into())] open: MaybeSignal<bool>,
    /// The callback to close the dialog.
    #[prop(into, optional)] close: Option<Callback<(), ()>>,
    /// The title of the dialog.
    #[prop(into)] title: TextProp,
    /// The message of the dialog.
    #[prop(into)] msg: TextProp,
) -> impl IntoView {
    view! {
        <Show when=move || open.get()>
            <div class="dialog">
                <dialog
                    open=open.get()
                    class=kind.class()
                >
                    <div class="dialog-header">{
                        let title = title.clone();
                        move || title.clone()
                    }</div>
                    <div class="dialog-main">{
                        let msg = msg.clone();
                        move || msg.clone()
                    }</div>
                    <div class="dialog-footer">
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
