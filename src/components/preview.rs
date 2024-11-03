use leptos::*;
use leptos_meta::*;

use crate::NovaFormContext;

use super::BaseContext;

pub fn start_preview(form_id: &str) {
    let base_context = expect_context::<BaseContext>();

    js_sys::eval(&format!(r#"
            if (document.getElementById("preview-wrapper").classList.contains("visible")) {{
                var preview = document.getElementById("preview");
                preview.innerHTML = "";
                var paged = new window.Paged.Previewer();
                window.previewer = paged;
                paged.preview(
                    '<div id="print">' + document.getElementById("{}").innerHTML + '</div>',
                    ["{}"],
                    preview
                );
            }}
        "#,
        form_id,
        base_context.resolve_path("/print.css")
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
                let scaleFactor =  Math.min(1, (preview.parentElement.offsetWidth / preview.scrollWidth));
                if (scaleFactor < 1) {
                    preview.style.position = "absolute";
                    preview.style.width = "210mm";
                    preview.style.transformOrigin = "top left";
                    preview.style.transform = "scale(" + scaleFactor + ")";
                    preview.style.marginRight = -210 * (1 - scaleFactor) + "mm";
                    preview.style.marginBottom = -preview.scrollHeight * (1 - scaleFactor) + "px";
                    preview.parentElement.style.height = preview.scrollHeight * scaleFactor + "px";
                } else {
                    preview.removeAttribute("style");
                    preview.parentElement.removeAttribute("style");
                }
            }

            window.PagedConfig = {
                auto: false
            };

            window.addEventListener("DOMContentLoaded", function() {
                const previewObserver = new MutationObserver(function(mutations) {
                    mutations.forEach(function(mutation) {
                        resizePreview();
                    });
                });
                
                previewObserver.observe(document.getElementById("preview"), {
                    childList: true,
                    subtree: true
                });
            });
            
            window.addEventListener("resize", resizePreview);
        "#</Script>
        <Script src="https://unpkg.com/pagedjs/dist/paged.polyfill.js"></Script>

        <div id="preview-wrapper" class=move || if nova_form_context.is_preview_mode() { "visible" } else { "hidden" }>
            <div
                id="preview"
            ></div>
        </div>
    }
}
