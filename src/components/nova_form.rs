use ev::SubmitEvent;
use leptos::*;
use leptos_i18n::*;
use leptos_router::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use server_fn::{
    client::Client, codec::PostUrl, error::NoCustomError, request::ClientReq, ServerFn,
};
use std::{fmt::Debug, marker::PhantomData, str::FromStr};
use thiserror::Error;

use crate::{
    FormDataSerialized, IconButton, IconSelect, InputsContext, Modal, ModalKind, PagesContext,
    QueryString, TriggerValidation,
};

#[derive(Error, Debug, Clone, Copy)]
enum SubmitError {
    #[error("the form contains errors")]
    ValidationError,
    #[error("the form contains errors")]
    ParseError,
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
    #[prop(into)] bind_meta_data: QueryString,
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
    let form_data_serialized = FormDataSerialized::from(form_data);

    provide_context(bind);
    provide_context(form_data_serialized.clone());

    let (preview_mode, set_preview_mode) = create_signal(false);
    let (submit_state, set_submit_state) = create_signal(SubmitState::Initial);
    let (trigger_validation, set_trigger_validation) = create_signal(TriggerValidation::None);

    let on_submit_value = on_submit.value();
    create_effect(move |_| match on_submit_value.get() {
        Some(Ok(_)) => set_submit_state.set(SubmitState::Success),
        Some(Err(_)) => set_submit_state.set(SubmitState::Error(SubmitError::ServerError)),
        None => {}
    });

    let (pages_context, set_pages_context) = create_signal(PagesContext::default());
    provide_context((pages_context, set_pages_context));

    let (inputs_context, set_inputs_context) =
        create_signal(InputsContext::new(trigger_validation));
    provide_context((inputs_context, set_inputs_context));

    let children = children();

    let pages = pages_context
        .get_untracked()
        .pages()
        .iter()
        .map(|page| (page.id(), page.label().clone()))
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
                },
                None => match id.language.as_str() {
                    "en" => "🇺🇸",
                    "de" => "🇩🇪",
                    "fr" => "🇫🇷",
                    "it" => "🇮🇹",
                    "es" => "🇪🇸",
                    _ => "",
                },
            };
            (
                *locale,
                if region_str.is_empty() {
                    format!("{}", language_str)
                } else {
                    format!("{} {}", region_str, language_str)
                }
                .into(),
            )
        })
        .collect::<Vec<_>>();

    let value = on_submit.value();

    let on_submit_inner = {
        move |ev: SubmitEvent| {
            if ev.default_prevented() {
                return;
            }
            ev.prevent_default();

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
            let do_submit = ev
                .submitter()
                .unwrap()
                .get_attribute("type")
                .map(|attr| attr == "submit")
                .unwrap_or(false);
            if !do_submit {
                return;
            }

            logging::log!("triggering validation");
            set_trigger_validation.set(TriggerValidation::All);
            if inputs_context.get().has_errors() {
                logging::log!("inputs have errors: {:?}", inputs_context.get());
                set_submit_state.set(SubmitState::Error(SubmitError::ValidationError));
                return;
            }

            let data = ServFn::from_event(&ev);
            if let Err(err) = data {
                logging::log!("error: {err}, {:?}", inputs_context.get());
                set_submit_state.set(SubmitState::Error(SubmitError::ParseError));
                return;
            }

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
                        value.set(Some(Err(ServerFnError::Serialization(err.to_string()))));
                    });
                }
            }
        }
    };

    view! {
        <form id="nova-form" action="" on:submit=on_submit_inner class=move || if preview_mode.get() { "hidden" } else { "edit" }>
            {children}
            <input type="hidden" name=bind_meta_data.add_key("locale") value={move || i18n.get_locale().to_string()} />
            <div>
                <IconButton
                    label="Previous Page"
                    icon="arrow_back"
                    on:click = move |_| set_pages_context.update(|pages_context| pages_context.prev())
                    disabled=Signal::derive(move || pages_context.get().is_first_selected()) />
                <IconButton label="Next Page" icon="arrow_forward"
                    on:click = move |_| {
                        set_pages_context.update(|pages_context| pages_context.next());
                     }
                    disabled=Signal::derive(move || pages_context.get().is_last_selected()) />
            </div>
        </form>

        <iframe scrolling="no" class=move || if !preview_mode.get() { "hidden" } else { "edit" } id="preview"></iframe>

        <script>r#"
            function isIframe() {
                return window.self !== window.top;
            }

            function preparePreview() {
                // Populate the values to the value attributes.
                document.querySelectorAll("input").forEach((input) => {
                    input.setAttribute("value", input.value);
                });

                // Populate the preview iframe with the current document.
                const preview = document.getElementById("preview");
                preview.srcdoc = document.documentElement.outerHTML;
            }

            function preparePreviewInsideIframe() {
                // Add the paged.js polyfill.
                let script = document.createElement("script");
                script.src = "https://unpkg.com/pagedjs/dist/paged.polyfill.js";
                document.head.appendChild(script);

                // Hide the body while the preview is being prepared.
                document.body.style.visibility = "hidden";
                document.body.style.backgroundColor = "white";
                window.PagedConfig = {
                    after: (flow) => {
                        document.body.removeAttribute("style");
                        parent.resizeIframe();
                    },
                };
                
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
                let scaleFactor =  Math.min(1, (window.innerWidth / preview.contentWindow.document.body.scrollWidth));
                if (scaleFactor < 1) {
                    preview.style.width = "210mm";
                    preview.style.transformOrigin = "top left";
                    preview.style.transform = "scale(" + scaleFactor + ")";
                    preview.style.marginRight = -210 * (1 - scaleFactor) + "mm";
                    preview.style.height = preview.contentWindow.document.body.scrollHeight + "px";
                    preview.style.marginBottom = -preview.contentWindow.document.body.scrollHeight * (1 - scaleFactor) + "px";
                } else {
                    preview.removeAttribute("style");
                    preview.style.height = preview.contentWindow.document.body.scrollHeight + "px";
                }

            }
        "#</script>

        <aside id="nova-form-actions" class="ui">
            <Show when=move || { pages_context.get().len() > 1 && !preview_mode.get() } >
                <IconSelect
                    id="menu"
                    label="Menu"
                    icon="menu"
                    values=pages.clone()
                    value=move || pages_context.get().selected().expect("page index out of bounds")
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

            <IconButton
                button_type="submit"
                label="Submit"
                icon="send"
                form="nova-form" />


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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaData {
    pub locale: String,
}

#[macro_export]
macro_rules! init_nova_forms {
    () => {
        // Initializes the locales for the form.
        leptos_i18n::load_locales!();
        use i18n::*;

        #[component]
        pub fn NovaFormContextProvider(
            #[prop(optional)] meta_data: Option<MetaData>,
            children: leptos::Children,
        ) -> impl IntoView {
            use std::str::FromStr;

            // Provides context that manages stylesheets, titles, meta tags, etc.
            leptos_meta::provide_meta_context();

            view! {
                <I18nContextProvider>
                    {
                        let i18n = use_i18n();

                        // Sets the locale from the meta data.
                        if let Some(meta_data) = meta_data {
                            i18n.set_locale(i18n::Locale::from_str(meta_data.locale.as_str()).unwrap());
                        }

                        view! {
                            <FormContainer title=t!(i18n, nova_forms) subtitle=t!(i18n, demo_form) logo="/logo.png">
                                {children()}
                            </FormContainer>
                        }
                    }
                </I18nContextProvider>
            }
        }
    };
}
