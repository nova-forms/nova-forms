use leptos::*;
use leptos_meta::*;

use crate::{NovaFormContext, NovaFormsContext};

pub fn start_preview(form_id: &str) {
    let nova_forms_context = expect_context::<NovaFormsContext>();

    js_sys::eval(&format!(r#"
            var preview = document.getElementById("preview");
            preview.innerHTML = "";
            var paged = new window.Paged.Previewer();
            window.previewer = paged;
            paged.preview(
                '<div id="print">' + document.getElementById("{}").innerHTML + '</div>',
                ["{}print.css"],
                preview
            );
        "#,
        form_id,
        nova_forms_context.base_url
    )).ok();
}

pub fn stop_preview(form_id: &str) {
    js_sys::eval(&format!(r#"
        var preview = document.getElementById("{}");
        window.previewer.removeStyles(preview);
    "#, form_id)).ok();
}

#[component]
pub fn Preview() -> impl IntoView {
    let nova_form_context = expect_context::<NovaFormContext>();

    view! {
        <Script>r#"
            window.styles = [];

            function resizePreview() {
                const preview = document.getElementById("preview");
                let scaleFactor =  Math.min(1, (window.innerWidth / preview.scrollWidth));
                if (scaleFactor < 1) {
                    preview.style.width = "210mm";
                    preview.style.transformOrigin = "top left";
                    preview.style.transform = "scale(" + scaleFactor + ")";
                    preview.style.marginRight = -210 * (1 - scaleFactor) + "mm";
                    preview.style.marginBottom = -preview.scrollHeight * (1 - scaleFactor) + "px";
                } else {
                    preview.removeAttribute("style");
                }
            }

            window.PagedConfig = {
                auto: false,
                after: () => {
                    resizePreview();
                }
            };
            
            window.addEventListener("resize", resizePreview);
        "#</Script>
        <Script src="https://unpkg.com/pagedjs/dist/paged.polyfill.js"></Script>

        <div
            id="preview"
            class=move || if nova_form_context.is_preview_mode() { "visible" } else { "hidden" }
        ></div>
    }
}
