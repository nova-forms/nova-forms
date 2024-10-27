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
    children: Children,
) -> impl IntoView {
    view! {
        <header>
            <img id="logo" src=logo />
            <div id="name">
                <span id="title">{title.clone()}</span>
                <br />
                <span id="subtitle">{subtitle}</span>
            </div>
        </header>
        <nav></nav>
        <main>{children()}</main>
        <footer>
            <span>{title}</span>
        </footer>
    }
}
