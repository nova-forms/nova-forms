use leptos::*;
use leptos_meta::*;

#[component]
pub fn FormContainer(
    #[prop(into)] logo: String,
    #[prop(into)] title: TextProp,
    #[prop(into)] subtitle: TextProp,
    children: Children,
) -> impl IntoView {
    view! {
        <Link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:opsz,wght,FILL,GRAD@24,400,1,0" />
        <header>
            <img id="logo" src=logo/>
            <div id="name">
                <span id="title">{title.clone()}</span><br/>
                <span id="subtitle">{subtitle}</span>
            </div>
        </header>
        <nav></nav>
        <main>
            {children()}
        </main>
        <footer>
            <span>{title}</span>
        </footer>
    }
}
