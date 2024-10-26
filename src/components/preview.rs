use leptos::*;

use crate::NovaFormContext;

pub fn prepare_preview() {
    js_sys::eval(r#"
        preparePreview();
    "#).ok();
}

#[component]
pub fn Preview() -> impl IntoView {
    let nova_form_context = expect_context::<NovaFormContext>();

    view! {
        <iframe
            scrolling="no"
            id="preview"
            class=move || if nova_form_context.is_preview_mode() { "visible" } else { "hidden" }
        ></iframe>

        <script>
            r#"
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
            "#
        </script>
    }
}