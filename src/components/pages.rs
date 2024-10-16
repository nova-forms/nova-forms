use leptos::*;

mod context {
    use std::{borrow::Cow, str::FromStr};
    use leptos::TextProp;

    #[derive(Debug, Clone)]
    pub struct PageData {
        id: PageId,
        label: TextProp,
    }

    impl PageData {
        pub fn new(id: PageId, label: TextProp) -> Self {
            Self { id, label }
        }

        pub fn id(&self) -> PageId {
            self.id.clone()
        }

        pub fn label(&self) -> TextProp {
            self.label.clone()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct PageId(Cow<'static, str>);

    impl ToString for PageId {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    impl FromStr for PageId {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(PageId(Cow::Owned(s.to_owned())))
        }
    }
    
    impl PageId {
        pub fn new(id: &'static str) -> Self {
            Self(Cow::Borrowed(id))
        }
    }

    #[derive(Debug, Clone)]
    pub struct PageContext {
        id: PageId,
    }

    impl PageContext {
        pub fn new(id: PageId) -> Self {
            Self { id }
        }

        pub fn id(&self) -> PageId {
            self.id.clone()
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct PagesContext {
        pages: Vec<PageData>,
        selected: usize,
    }

    impl PagesContext {
        pub fn register(&mut self, data: PageData) {
            self.pages.push(data);
        }

        pub fn is_selected(&self, id: PageId) -> bool {
            self.pages
                .iter()
                .position(|t| t.id == id)
                .map(|idx| idx == self.selected)
                .unwrap_or(false)
        }

        pub fn is_first_selected(&self) -> bool {
            self.pages.is_empty() || self.selected == 0
        }

        pub fn is_last_selected(&self) -> bool {
            self.pages.is_empty() || self.selected == self.pages.len() - 1
        }

        pub fn next(&mut self) {
            if self.selected + 1 < self.pages.len() {
                self.selected += 1;
            }
        }

        pub fn prev(&mut self) {
            if self.selected > 0 {
                self.selected -= 1;
            }
        }

        pub fn select(&mut self, id: PageId) {
            if let Some(idx) = self.pages.iter().position(|t| t.id == id) {
                self.selected = idx;
            }
        }

        pub fn len(&self) -> usize {
            self.pages.len()
        }

        pub fn selected(&self) -> Option<PageId> {
            self.pages
                .get(self.selected)
                .map(|tab_data| tab_data.id.clone())
        }

        pub fn pages(&self) -> &[PageData] {
            self.pages.as_slice()
        }
    }
}

pub(crate) use context::*;

/*
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
*/

#[component]
pub fn Page(id: &'static str, #[prop(into)] label: TextProp, children: Children) -> impl IntoView {
    let id = PageId::new(id);

    let (pages_context, set_pages_context) =
        expect_context::<(ReadSignal<PagesContext>, WriteSignal<PagesContext>)>();

    set_pages_context.update(|pages_context| {
        pages_context.register(PageData::new(id.clone(), label))
    });

    view! {
        <Provider value=PageContext::new(id.clone())>
            <div class=move || if pages_context.get().is_selected(id.clone()) { "page selected" } else { "page hidden" } >
                {children()}
            </div>
        </Provider>

    }
}
