use ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use serde::{de::DeserializeOwned, Serialize};
use server_fn::{
    client::Client, codec::PostUrl, error::NoCustomError, request::ClientReq, ServerFn,
};
use std::marker::PhantomData;

use crate::{FormDataSerialized, Icon, IconButton, Pages, QueryString};


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
    
    let form_data = FormDataSerialized::from(form_data);

    let (preview_mode, set_preview_mode) = create_signal(false);

    provide_context(bind);
    provide_context(form_data.clone());
   
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
 
            ev.prevent_default();

            match ServFn::from_event(&ev) {
                Ok(new_input) => {
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

    view! {
        /*
        <Form action="POST" on:submit = on_submit_inner>
            {children()}
        </Form>
        <Show when=not_print_mode>
            <iframe id="preview" src=format!("/?mode=print&data={}", form_data.clone().to_query_string())></iframe>
        </Show>
        <Show when=print_mode>
            <style>r#"
                form > *, header, footer {
                    display: none;
                }
            "#</style>
            <script>r#"
                window.PagedConfig = {
                    after: () => {
                        const elems = document.querySelectorAll("form > *");
                        for (let i = 0; i < elems.length; i++) {
                            elems[i].style.display = "block";
                        }
                    },
                };
            "#</script>
            <script src="https://unpkg.com/pagedjs/dist/paged.polyfill.js"></script>
        </Show>
        */
        /*<script src="https://unpkg.com/pagedjs/dist/paged.polyfill.js"></script>*/

     
        
        <form id="nova-form" action="" on:submit=on_submit_inner class=move || if preview_mode.get() { "hidden" } else { "edit" }>
            {children()}
        </form>

        <iframe class=move || if !preview_mode.get() { "hidden" } else { "edit" } id="preview"></iframe>
        //<div id="ruler" style="width: 210mm; height: 297mm; display: block"></div>

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
                /*
                const ruler = document.getElementById("ruler");
                let vw = Math.max(document.documentElement.clientWidth || 0, window.innerWidth || 0)
                console.log(ruler.offsetWidth, vw, (ruler.offsetWidth / vw));
                preview.style.transform = "scale(" + Math.min(1, (vw / ruler.offsetWidth)) + ")";
                preview.style.transformOrigin = "top left";
                */
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
            }
        "#</script>

        <aside id="nova-form-actions">


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

            //<IconButton button_type="submit" label="Submit" icon="send" form="nova-form"/>
            <button type="submit" class="icon-button" form="nova-form" >
                <Icon label="Submit" icon="send" />
            </button>

        
        </aside>
        

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
