use ev::SubmitEvent;
use leptos::*;
use leptos_i18n::*;
use leptos_router::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use server_fn::{
    client::Client, codec::PostUrl, error::NoCustomError, request::ClientReq, ServerFn,
};
use strum::Display;
use time::UtcOffset;
use ustr::Ustr;
use std::{fmt::Debug, marker::PhantomData, str::FromStr};
use thiserror::Error;

use crate::{
    local_utc_offset, use_translation, FormDataSerialized, InputsContext, Modal, DialogKind, PagesContext, Preview, QueryString, Toolbar, ToolbarLocaleSelect, ToolbarPageSelect, ToolbarPreviewButton, ToolbarSubmitButton
};

use super::{InputData, PageContext};

#[derive(Clone, Copy, Debug, Display)]
pub enum Translation {
    Submit,
    Preview,
    Edit,
    Language,
    Menu,
}

#[derive(Error, Debug, Clone)]
pub enum SubmitError {
    #[error("the form contains errors")]
    ValidationError,
    #[error("the form contains errors")]
    ParseError,
    #[error("a server error occurred: {0}")]
    ServerError(ServerFnError),
}

#[derive(Clone, Display)]
pub enum SubmitState {
    Initial,
    Pending,
    Error(SubmitError),
    Success,
}

pub(crate) type Version = u64;

#[derive(Debug, Clone, Copy)]
pub struct NovaFormContext {
    form_id: Ustr,
    preview: RwSignal<bool>,
    trigger_validation: RwSignal<Version>,
    inputs: RwSignal<InputsContext>,
    render: RwSignal<bool>,
}

impl NovaFormContext {
    pub fn trigger_validation(&self) -> Result<(), ()> {
        self.trigger_validation.update(|v| *v += 1);

        if let Some(input) = self.inputs.get().has_errors() {
            if let Some(pages_context) = use_context::<RwSignal<PagesContext>>() {
                pages_context.update(|pages_context| pages_context.select(input.page_id.unwrap()));
            }
            return Err(());
        }

        Ok(())
    }

    pub fn is_render_mode(&self) -> bool {
        self.render.get() || self.preview.get()
    }

    pub fn is_preview_mode(&self) -> bool {
        self.preview.get()
    }

    pub fn is_edit_mode(&self) -> bool {
        !self.is_preview_mode()
    }

    pub fn preview_mode(&self) {
        self.render.set(true);
        self.preview.set(true);
    }

    pub fn edit_mode(&self) {
        self.render.set(false);
        self.preview.set(false);
    }

    pub fn form_id(&self) -> &str {
        self.form_id.as_str()
    }

    pub(crate) fn version(&self) -> u64 {
        self.trigger_validation.get_untracked()
    }

    pub(crate) fn set_error(&self, qs: &QueryString, has_error: bool) {
        self.inputs.update(|inputs| {
            inputs.set_error(&qs, has_error);
        });
    }

    pub(crate) fn deregister_input(&self, qs: QueryString) {
        self.inputs.update(|inputs| {
            inputs.deregister(&qs);
        });
    }

    pub(crate) fn register_input(&self, qs: QueryString, label: TextProp) -> Signal<bool> {
        self.inputs.update(|inputs| {
            inputs.register(qs, InputData {
                page_id: use_context::<PageContext>().map(|page| page.id()),
                label,
                has_error: false,
                version: self.version(),
            });
        });

        let version = self.version();
        let trigger_validation = self.trigger_validation;

        Signal::derive(move || trigger_validation.get() > version)
    }
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
    #[prop(optional)] render: bool,
) -> impl IntoView
where
    F: Default + Clone + Serialize + Debug + 'static,
    ServFn: DeserializeOwned + Serialize
        + ServerFn<InputEncoding = PostUrl, Error = NoCustomError, Output = ()>
        + 'static,
    <<ServFn::Client as Client<ServFn::Error>>::Request as ClientReq<ServFn::Error>>::FormData:
        From<web_sys::FormData>,
    L: Locale + 'static,
    <L as FromStr>::Err: Debug,
    K: LocaleKeys<Locale = L> + 'static,
{
    if cfg!(debug_assertions) {
        logging::log!("debug mode enabled, prefilling input fields with valid data");
    }
    //let (local_storage, set_local_storage, _) = use_local_storage::<Option<FormDataSerialized>, FromToStringCodec>("form_data");
    //logging::log!("Form data local storage: {:?}, provided {:?}", local_storage.get(), form_data);

    let form_data_serialized = FormDataSerialized::from(form_data);

    provide_context(bind.clone());
    provide_context(form_data_serialized.clone());

    let preview = create_rw_signal(false);
    let render = create_rw_signal(render);
    let form_id = Ustr::from("nova-form");
    let inputs = create_rw_signal(InputsContext::new());
    let trigger_validation = create_rw_signal(0);
    let nova_form_context = NovaFormContext { preview, form_id, trigger_validation, inputs, render };
    provide_context(nova_form_context);

    let (submit_state, set_submit_state) = create_signal(SubmitState::Initial);

    let on_submit_value = on_submit.value();
    create_effect(move |_| match on_submit_value.get() {
        Some(Ok(_)) => set_submit_state.set(SubmitState::Success),
        Some(Err(err)) => set_submit_state.set(SubmitState::Error(SubmitError::ServerError(err))),
        None => {}
    });

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

            //trigger_validation.set(Instant::now());
            /*if let Some(input_data) = inputs_context.get().has_errors() {
                set_submit_state.set(SubmitState::Error(SubmitError::ValidationError));

                if let Some(pages_context) = use_context::<RwSignal<PagesContext>>() {
                    pages_context.update(|pages_context| pages_context.select(input_data.page_id.clone()));
                }
                return;
            }*/
            if nova_form_context.trigger_validation().is_err() {
                set_submit_state.set(SubmitState::Error(SubmitError::ValidationError));
                return;
            }



            match ServFn::from_event(&ev) {
                Ok(new_input) => {
                    //let form_data = FormDataSerialized::from(&new_input);
                    //logging::log!("Form data: {form_data:?}");
                    //set_local_storage.set(form_data.clone());
                    set_submit_state.set(SubmitState::Pending);
                    on_submit.dispatch(new_input);
                }
                Err(err) => {
                    set_submit_state.set(SubmitState::Error(SubmitError::ParseError));
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
        <form
            id=form_id.as_str()
            novalidate
            action=""
            on:submit=on_submit_inner
            class=move || if preview.get() { "hidden" } else { "visible" }
        >
            {children()}

            // Add the metadata using hidden fields.
            <input
                type="hidden"
                name=bind_meta_data.clone().add_key("locale")
                value=move || i18n.get_locale().to_string()
            />
            <input
                type="hidden"
                name=bind_meta_data.clone().add_key("local_utc_offset")
                value=move || local_utc_offset().to_string()
            />
        </form>

        <Preview/>

        <Toolbar>
            <ToolbarPageSelect />
            <ToolbarLocaleSelect i18n=i18n />
            <ToolbarPreviewButton />
            <ToolbarSubmitButton />
        </Toolbar>

        
        <Modal
            open=Signal::derive(move || matches!(submit_state.get(), SubmitState::Pending))
            kind=DialogKind::Info
            title={use_translation(Translation::Submit)}
            msg={use_translation(submit_state.get())}
        />

        <Modal
            open=Signal::derive(move || matches!(submit_state.get(), SubmitState::Error(_)))
            kind=DialogKind::Error
            title={use_translation(Translation::Submit)}
            msg={use_translation(submit_state.get())}
            close=move |()| set_submit_state.set(SubmitState::Initial)
        />
    
        <Modal
            open=Signal::derive(move || matches!(submit_state.get(), SubmitState::Success))
            kind=DialogKind::Success
            title={use_translation(Translation::Submit)}
            msg={use_translation(submit_state.get())}
            close=move |()| set_submit_state.set(SubmitState::Initial)
        />
        
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaData {
    pub locale: String,
    pub local_utc_offset: UtcOffset,
}

#[macro_export]
macro_rules! init_nova_forms {
    () => {
        // Initializes the locales for the form.
        leptos_i18n::load_locales!();
        use i18n::*;

        #[component]
        pub fn NovaFormsContextProvider(
            #[prop(optional)] meta_data: Option<MetaData>,
            #[prop(optional, into)] base_url: Option<String>,
            children: leptos::Children,
        ) -> impl IntoView {
            use std::str::FromStr;

            // Provides context that manages stylesheets, titles, meta tags, etc.
            leptos_meta::provide_meta_context();

            let base_url = {
                if let Some(mut base_url) = base_url {
                    if !base_url.ends_with('/') {
                        base_url = format!("{}/", base_url);
                    }
                    if !base_url.starts_with('/') {
                        base_url = format!("/{}", base_url);
                    }
                    base_url
                } else {
                    String::from("/")
                }
            };

            provide_context::<NovaFormsContext>(NovaFormsContext {
                base_url: base_url.clone(),
            });

            view! {
                // Injects a stylesheet into the document <head>.
                // id=leptos means cargo-leptos will hot-reload this stylesheet.
                <Stylesheet id="leptos" href={format!("{}pkg/app.css", base_url)} />

                <Link
                    rel="stylesheet"
                    href="https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:opsz,wght,FILL,GRAD@24,400,1,0"
                />

                <I18nContextProvider>
                    {
                        let i18n = use_i18n();

                        // Sets the locale from the meta data.
                        if let Some(meta_data) = meta_data {
                            i18n.set_locale(i18n::Locale::from_str(meta_data.locale.as_str()).unwrap());
                        }

                        view! {
                            {children()}
                        }
                    }
                </I18nContextProvider>
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct NovaFormsContext {
    pub base_url: String,
}

