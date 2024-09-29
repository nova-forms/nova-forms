use leptos::*;

#[derive(Debug, Clone)]
struct TabData {
    id: TabId,
    label: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TabId(&'static str);

#[derive(Debug, Clone, Default)]
struct PagesContext {
    tabs: Vec<TabData>,
    selected: usize,
}

impl PagesContext {
    fn register(&mut self, data: TabData) {
        self.tabs.push(data);
    }

    fn is_selected(&self, id: TabId) -> bool {
        self.tabs.iter().position(|t| t.id == id).map(|idx| idx == self.selected).unwrap_or(false)
    }

    fn first_selected(&self) -> bool {
        self.selected == 0 && !self.tabs.is_empty()
    }

    fn last_selected(&self) -> bool {
        self.selected == self.tabs.len() - 1 && !self.tabs.is_empty()
    }

    fn next(&mut self) {
        if self.selected + 1 < self.tabs.len() {
            self.selected += 1;
        }
    }

    fn prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    fn select(&mut self, id: TabId) {
        if let Some(idx) = self.tabs.iter().position(|t| t.id == id) {
            self.selected = idx;
        }
    }
    /*
    fn len(&self) -> usize {
        self.tabs.len()
    }

    fn selected(&self) -> usize {
        self.selected
    }
    */
}

#[component]
pub fn Pages(children: Children) -> impl IntoView {
    let (pages_context, set_pages_context) = create_signal(PagesContext::default());
    
    provide_context((pages_context, set_pages_context));

    let children = children();

    view! {
        <div class="pages">
            <nav>
                <For 
                    each=move || pages_context.get().tabs.clone().into_iter().enumerate()
                    key=|(_, tab)| tab.id
                    children = move |(_i, tab)| {
                        view! {
                            <button type="button" on:click = move |_| set_pages_context.update(|pages_context| pages_context.select(tab.id)) disabled=move || pages_context.get().is_selected(tab.id)>{tab.label}</button>
                        }
                    }
                />
            </nav>
            {children}
            <div>
                <Show when=move || !pages_context.get().first_selected() >
                    <button type="button" on:click = move |_| set_pages_context.update(|pages_context| pages_context.prev()) >"Previous Page"</button>
                </Show>
                <Show when=move || !pages_context.get().last_selected() >
                    <button type="button" on:click = move |_| set_pages_context.update(|pages_context| pages_context.next()) >"Next Page"</button>
                </Show>
            </div>

        </div>
    }
}

#[component]
pub fn Page(
    id: &'static str,
    #[prop(into)] label: String,
    children: Children
) -> impl IntoView {
    let id = TabId(id);

    let (pages_context, set_pages_context) = expect_context::<(ReadSignal<PagesContext>, WriteSignal<PagesContext>)>();

    set_pages_context.update(|pages_context| pages_context.register(TabData { id, label: label.clone() }));

    view! {
        <div class=move || if pages_context.get().is_selected(id) { "page selected" } else { "page hidden" } >
            {children()}
        </div>
    }
}
