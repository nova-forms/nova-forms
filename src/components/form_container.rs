use leptos::*;

#[component]
pub fn FormContainer(
    #[prop(into)] logo: String,
    #[prop(into)] title: String,
    #[prop(into)] subtitle: String,
    children: Children,
) -> impl IntoView {
    view! {
        //<script src="https://unpkg.com/pagedjs/dist/paged.polyfill.js"></script>
        <header>
            <img id="logo" src=logo/>
            <div id="name">
                <span id="title">{&title}</span><br/>
                <span id="subtitle">{&subtitle}</span>
            </div>
        </header>
        <nav></nav>
        <main>
            {children()}
        </main>
        <footer>
            <span>{&title}</span>
        </footer>
    }
}
