use ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use serde::{de::DeserializeOwned, Serialize};
use server_fn::{
    client::Client, codec::PostUrl, error::NoCustomError, request::ClientReq, ServerFn,
};
use std::{fmt::Debug, marker::PhantomData, str::FromStr};
use thiserror::Error;
use leptos_i18n::*;

use crate::{FormDataSerialized, IconButton, IconSelect, Modal, ModalKind, PagesContext, QueryString};

#[derive(Error, Debug, Clone, Copy)]
enum SubmitError {
    #[error("the form contains errors")]
    ValidationError,
    #[error("a server error occurred")]
    ServerError,
}

#[derive(Debug, Clone, Copy)]
enum SubmitState {
    Initial,
    Pending,
    Error(SubmitError),
    Success,
}

#[component]
pub fn NovaForm<F, ServFn, L, K>(
    #[prop(optional)] form_data: F,
    on_submit: Action<ServFn, Result<(), ServerFnError>>,
    #[prop(into)] bind: QueryString,
    #[prop(optional)] _arg: PhantomData<ServFn>,
    i18n: I18nContext<L, K>,
    children: Children,
) -> impl IntoView
where
    F: Default + Serialize + 'static,
    ServFn: DeserializeOwned
        + ServerFn<InputEncoding = PostUrl, Error = NoCustomError, Output = ()>
        + 'static,
    <<ServFn::Client as Client<ServFn::Error>>::Request as ClientReq<ServFn::Error>>::FormData:
        From<web_sys::FormData>,
    L: Locale + 'static,
    <L as FromStr>::Err: Debug,
    K: LocaleKeys<Locale = L> + 'static,
{

    let form_data = FormDataSerialized::from(form_data);

    provide_context(bind);
    provide_context(form_data.clone());

    let (preview_mode, set_preview_mode) = create_signal(false);
    let (submit_state, set_submit_state) = create_signal(SubmitState::Initial);

    let on_submit_value = on_submit.value();
    create_effect(move |_| {
        match on_submit_value.get() {
            Some(Ok(_)) => set_submit_state.set(SubmitState::Success),
            Some(Err(_)) => set_submit_state.set(SubmitState::Error(SubmitError::ServerError)),
            None => {}
        }
    });
   
    let version = on_submit.version();
    let value = on_submit.value();

    let on_submit_inner = {
        move |ev: SubmitEvent| {
            if ev.default_prevented() {
                return;
            }

            // <button formmethod="dialog"> should *not* dispatch the action, but should be allowed to
            // just bubble up and close the <dialog> naturally
            let is_dialog = ev
                .submitter()
                .and_then(|el| el.get_attribute("formmethod"))
                .as_deref()
                == Some("dialog");
            if is_dialog {
                return;
            }

            // Do not submit the form if the submit button is not the one that was clicked.
            let do_submit = ev.submitter().unwrap().get_attribute("type").map(|attr| attr == "submit").unwrap_or(false);
            if !do_submit {
                ev.prevent_default();
                return;
            }

            let data = ServFn::from_event(&ev);
            if let Err(err) = data {
                println!("error: {err}");
                set_submit_state.set(SubmitState::Error(SubmitError::ValidationError));
                ev.prevent_default();
                return;
            }
 
            ev.prevent_default();

            match ServFn::from_event(&ev) {
                Ok(new_input) => {
                    set_submit_state.set(SubmitState::Pending);
                    on_submit.dispatch(new_input);
                }
                Err(err) => {
                    logging::error!(
                        "Error converting form field into server function \
                         arguments: {err:?}"
                    );
                    batch(move || {
                        value.set(Some(Err(ServerFnError::Serialization(
                            err.to_string(),
                        ))));
                        version.update(|n| *n += 1);
                    });
                }
            }
        }
    };

    let (pages_context, set_pages_context) = create_signal(PagesContext::default());
    
    provide_context((pages_context, set_pages_context));

    let children = children();

    let pages = pages_context.get().pages()
        .iter()
        .map(|tab| {
            (tab.id.clone(), tab.label.clone())
        })
        .collect::<Vec<_>>();
    
    let locales = L::get_all()
        .iter()
        .map(|locale| {
            let id = &locale.as_icu_locale().id;
            let language_str = match id.language.as_str() {
                "en" => "English",
                "de" => "Deutsch",
                "fr" => "Français",
                "it" => "Italiano",
                "es" => "Español",
                other => other,
            };
            let region = id.region.as_ref();
            let region_str = match region {
                Some(region) => match region.as_str() {
                    "US" => "🇺🇸",
                    "GB" => "🇬🇧",
                    "DE" => "🇩🇪",
                    "CH" => "🇨🇭",
                    "FR" => "🇫🇷",
                    "IT" => "🇮🇹",
                    "ES" => "🇪🇸",
                    other => other,
                }
                None => match id.language.as_str() {
                    "en" => "🇺🇸",
                    "de" => "🇩🇪",
                    "fr" => "🇫🇷",
                    "it" => "🇮🇹",
                    "es" => "🇪🇸",
                    _ => "",
                },
            };
            (*locale, if region_str.is_empty() { format!("{}", language_str) } else { format!("{} {}", region_str, language_str) }.into())
        })
        .collect::<Vec<_>>();
    

    view! {    
        <form id="nova-form" action="" on:submit=on_submit_inner class=move || if preview_mode.get() { "hidden" } else { "edit" }>
            {children}
            <div>
                <Show when=move || !pages_context.get().is_first_selected() >
                    <IconButton label="Previous Page" icon="arrow_back" on:click = move |_| set_pages_context.update(|pages_context| pages_context.prev()) />
                </Show>
                <Show when=move || !pages_context.get().is_last_selected() >
                    <IconButton label="Next Page" icon="arrow_forward" on:click = move |_| set_pages_context.update(|pages_context| pages_context.next()) />
                </Show>
                /*<Show when=move || pages_context.get().is_last_selected() >
                    <IconButton button_type="submit" label="Submit" icon="send" />
                </Show>*/
            </div>
        </form>

        <iframe class=move || if !preview_mode.get() { "hidden" } else { "edit" } id="preview"></iframe>

        <script>r#"
            function isIframe() {
                return window.self !== window.top;
            }

            function preparePreview() {
                console.log("Preparing preview...");
                // Populate the values to the value attributes.
                document.querySelectorAll("input").forEach((input) => {
                    input.setAttribute("value", input.value);
                });

                // Populate the preview iframe with the current document.
                const preview = document.getElementById("preview");
                preview.srcdoc = document.documentElement.outerHTML;

                // Scale the preview to fit the screen.
                (new MutationObserver(resizeIframe)).observe(
                    preview.contentWindow.document.body,
                    { attributes: true, childList: true, subtree: true }
                );
            }

            function preparePreviewInsideIframe() {
                // Add the paged.js polyfill.
                var script = document.createElement("script");
                script.src = "https://unpkg.com/pagedjs/dist/paged.polyfill.js";
                document.head.appendChild(script);

                // Disable all form inputs.
                document.querySelectorAll("input").forEach((input) => {
                    input.disabled = true;
                });
            }

            if (isIframe()) {
                preparePreviewInsideIframe();
            } else {
                window.addEventListener("resize", resizeIframe);
            }

            function resizeIframe() {
                const preview = document.getElementById("preview");
                console.log("resizing iframe", window.innerWidth, preview.offsetWidth, Math.min(1, (window.innerWidth / preview.contentWindow.document.body.scrollWidth)), preview.contentWindow.document.body.scrollWidth);
                let scaleFactor =  Math.min(1, (window.innerWidth / preview.contentWindow.document.body.scrollWidth));
                if (scaleFactor < 1) {
                    preview.style.width = "210mm";
                    preview.style.height = "297mm";
                    preview.style.transformOrigin = "top left";
                    preview.style.transform = "scale(" + scaleFactor + ")";
                    preview.style.marginRight = -210 * (1 - scaleFactor) + "mm";
                    preview.style.marginBottom = -297 * (1 - scaleFactor) + "mm";
                } else {
                    preview.removeAttribute("style");
                }
            }
        "#</script>

        <aside id="nova-form-actions">


            <Show when=move || { pages_context.get().len() > 1 && !preview_mode.get() } >
                <IconSelect
                    id="menu"
                    label="Menu"
                    icon="menu"
                    values=pages.clone()
                    value=move || pages_context.get().selected().unwrap()
                    on_change=move |tab_id| set_pages_context.update(|pages_context| pages_context.select(tab_id)) />
            </Show>

            <Show when=move || !preview_mode.get() >
                <IconSelect
                    id="language"
                    label="Language"
                    icon="translate"
                    values=locales.clone()
                    value=move || i18n.get_locale()
                    on_change=move |locale| i18n.set_locale(locale) />
            </Show>
            


            //<IconButton label="Menu" icon="menu" on:click=move |_| { } />

            {
                move || if preview_mode.get() {
                    view! {
                        <IconButton label="Edit" icon="edit" on:click=move |_| {
                            set_preview_mode.set(false);
                        } />
                    }
                } else {
                    view! {
                        <IconButton label="Preview" icon="visibility" on:click=move |_| {
                            js_sys::eval("preparePreview();").ok();
                            set_preview_mode.set(true)
                        } />
                    }
                }
            }

            <IconButton button_type="submit" label="Submit" icon="send" form="nova-form"/>

        
        </aside>

        { move || match submit_state.get() {
            SubmitState::Initial => view! {}.into_view(),
            SubmitState::Pending => view! {
                <Modal kind=ModalKind::Info title="Submission" close=move |()| set_submit_state.set(SubmitState::Initial)>
                    "Your form is being submitted."
                </Modal>
            }.into_view(),
            SubmitState::Error(err) => view! {
                <Modal kind=ModalKind::Error title="Submission" close=move |()| set_submit_state.set(SubmitState::Initial)>
                    {format!("Your form could not be submitted: {err}.")}
                </Modal>
            }.into_view(),
            SubmitState::Success => view! {
                <Modal kind=ModalKind::Success title="Submission" close=move |()| set_submit_state.set(SubmitState::Initial)>
                    "Your form was successfully submitted."
                </Modal>
            }.into_view(),
        } }
        

        /*<IconButton label="Download" icon="download" on:click=move |_| {
            /*web_sys::window().and_then(|w| w.print().ok());*/
            js_sys::eval(r#"
                preparePreview();
                setTimeout(function() {
                    const preview = document.getElementById('preview');
                    preview.contentWindow.print();
                }, 5000);
            "#).ok();
        } />*/

    }
}
