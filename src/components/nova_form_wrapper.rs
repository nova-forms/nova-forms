use leptos::*;

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
    #[prop(into, optional)] footer: Option<Children>,
    children: Children,
) -> impl IntoView {
    view! {
        <header>
            <div class="content">
                {
                    if logo.ends_with(".svg") {
                        view! { <img id="logo" src=logo /> }
                    } else {
                        view! { <img id="logo" src=logo /> }
                    }
                }
                <div id="name">
                    <span id="title">{title.clone()}</span>
                    <br />
                    <span id="subtitle">{subtitle}</span>
                </div>
            </div>
        </header>
        <nav>
            <div class="content">
            </div>
        </nav>
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
