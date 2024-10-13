use std::{borrow::Cow, str::FromStr};

use leptos::*;

#[derive(Debug, Clone)]
pub(crate) struct PageData {
    pub(crate) id: PageId,
    pub(crate) label: TextProp,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PageId(Cow<'static, str>);

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

#[derive(Debug, Clone, Default)]
pub(crate) struct PagesContext {
    tabs: Vec<PageData>,
    selected: usize,
}

impl PagesContext {
    pub(crate) fn register(&mut self, data: PageData) {
        self.tabs.push(data);
    }

    pub(crate) fn is_selected(&self, id: PageId) -> bool {
        self.tabs
            .iter()
            .position(|t| t.id == id)
            .map(|idx| idx == self.selected)
            .unwrap_or(false)
    }

    pub(crate) fn is_first_selected(&self) -> bool {
        self.tabs.is_empty() || self.selected == 0
    }

    pub(crate) fn is_last_selected(&self) -> bool {
        self.tabs.is_empty() || self.selected == self.tabs.len() - 1
    }

    pub(crate) fn next(&mut self) {
        if self.selected + 1 < self.tabs.len() {
            self.selected += 1;
        }
    }

    pub(crate) fn prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub(crate) fn select(&mut self, id: PageId) {
        if let Some(idx) = self.tabs.iter().position(|t| t.id == id) {
            self.selected = idx;
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.tabs.len()
    }

    pub(crate) fn selected(&self) -> Option<PageId> {
        self.tabs
            .get(self.selected)
            .map(|tab_data| tab_data.id.clone())
    }

    pub(crate) fn pages(&self) -> &[PageData] {
        self.tabs.as_slice()
    }
}

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
    let id = PageId(Cow::Borrowed(id));

    let (pages_context, set_pages_context) =
        expect_context::<(ReadSignal<PagesContext>, WriteSignal<PagesContext>)>();

    set_pages_context.update(|pages_context| {
        pages_context.register(PageData {
            id: id.clone(),
            label,
        })
    });

    view! {
        <div class=move || if pages_context.get().is_selected(id.clone()) { "page selected" } else { "page hidden" } >
            {children()}
        </div>
    }
}
