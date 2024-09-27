use leptos::*;
use leptos_router::*;
use serde::{de::DeserializeOwned, Serialize};
use server_fn::{
    client::Client, codec::PostUrl, error::NoCustomError, request::ClientReq, ServerFn,
};
use std::marker::PhantomData;

use crate::{FormDataSerialized, QueryString};

#[component]
pub fn NovaForm<F, ServFn>(
    #[prop(optional)] form_data: F,
    on_submit: Action<ServFn, Result<(), ServerFnError>>,
    #[prop(into)] bind: QueryString,
    #[prop(optional)] _arg: PhantomData<ServFn>,
    children: Children,
) -> impl IntoView
where
    F: Default + Serialize + 'static,
    ServFn: DeserializeOwned
        + ServerFn<InputEncoding = PostUrl, Error = NoCustomError, Output = ()>
        + 'static,
    <<ServFn::Client as Client<ServFn::Error>>::Request as ClientReq<ServFn::Error>>::FormData:
        From<web_sys::FormData>,
{
    provide_context(bind);
    provide_context(FormDataSerialized::from(form_data));

    view! {
        <ActionForm action=on_submit>
            {children()}
        </ActionForm>
    }
}
