use leptos::*;

use crate::AppContext;

/// A container for a form.
/// Adds a header with a logo, title, and subtitle, as well as a footer with the title.
#[component]
pub fn NovaFormWrapper(
    /// The URL of the logo to display in the header.
    #[prop(into)] logo: String,
    /// The title to display in the header and footer.
    #[prop(into)] title: TextProp,
    /// The subtitle to display in the header.
    #[prop(into)] subtitle: TextProp,
    /// The footer to display at the bottom of the form.
    /// By default, there is no footer.
    #[prop(into, optional)] footer: Option<Children>,
    /// The nova form goes here.
    children: Children,
) -> impl IntoView {
    let base_context = expect_context::<AppContext>();

    view! {
        <header>
            <div class="content">  
                <img id="logo" src=base_context.resolve_path(logo) />
                <div id="name">
                    <span id="title">{title.clone()}</span>
                    <br />
                    <span id="subtitle">{subtitle}</span>
                </div>
            </div>
        </header>
        <main>
            <div class="content">
                {children()}
            </div>
        </main>
        {
            if let Some(footer) = footer {
                view! { 
                    <footer>
                        <div class="content">
                            <span>{footer()}</span>
                        </div>
                    </footer>
                }.into_view()
            } else {
                View::default()
            }
        }
        
    }
}
