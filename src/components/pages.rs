use leptos::*;
use crate::{ButtonGroup, Button};

mod context {
    use leptos::TextProp;
    use ustr::Ustr;
    use std::str::FromStr;

    #[derive(Debug, Clone)]
    pub struct PageData {
        id: PageId,
        label: TextProp,
        idx: usize,
    }

    impl PageData {
        fn new(id: PageId, label: TextProp, idx: usize) -> Self {
            Self { id, label, idx }
        }

        pub fn id(&self) -> PageId {
            self.id
        }

        pub fn label(&self) -> TextProp {
            self.label.clone()
        }

        pub fn idx(&self) -> usize {
            self.idx
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PageId(Ustr);

    impl ToString for PageId {
        fn to_string(&self) -> String {
            self.0.to_string()
        }
    }

    impl FromStr for PageId {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(PageId(Ustr::from(s)))
        }
    }

    impl PageId {
        pub fn new(id: &'static str) -> Self {
            Self(Ustr::from(id))
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
            self.id
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct PagesContext {
        pages: Vec<PageData>,
        selected: usize,
    }

    impl PagesContext {
        pub fn register(&mut self, label: TextProp, id: PageId) {
            self.pages.push(PageData::new(id, label, self.pages.len()));
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

#[component]
pub fn Pages(
    children: Children,
) -> impl IntoView
where
{

    let pages = create_rw_signal(PagesContext::default());
    provide_context(pages);

    let children = children();

    view! { <div class="pages">{children}</div> }
}


/// Creates a new page in the form.
#[component]
pub fn Page(
    /// The id of the page.
    id: &'static str,
    /// The label of the page.
    #[prop(into)] label: TextProp,
    /// The contents of the page.
    children: Children
) -> impl IntoView {
    let id = PageId::new(id);

    let pages_context = expect_context::<RwSignal<PagesContext>>();

    pages_context.update(|pages_context| pages_context.register(label, id));

    view! {
        <Provider value=PageContext::new(id)>
            <div class=move || {
                if pages_context.get().is_selected(id) { "page selected" } else { "page hidden" }
            }>{children()}</div>
        </Provider>
    }
}


/// Creates a new page in the form.
#[component]
pub fn PageStepper(
) -> impl IntoView {
    let pages_context = expect_context::<RwSignal<PagesContext>>();

    view! {
        <div class="stepper">
            <ButtonGroup>
                <Button
                    label="Previous Page"
                    icon="arrow_back"
                    on:click=move |_| pages_context.update(|pages| pages.prev())
                    disabled=Signal::derive(move || pages_context.get().is_first_selected())
                />
                <div class="stepper-spacer" />
                <For
                    each=move || {
                        let pages = pages_context.get().pages().iter().cloned().collect::<Vec<_>>();
                        pages
                    }
                    key=|page| page.id()
                    children=move |page| {
                        let page_id = page.id();
                        view! {
                            <button
                                class="icon-button stepper-page-number"
                                on:click=move |_| {
                                    pages_context.update(|pages_context| pages_context.select(page_id))
                                }
                                disabled=move || pages_context.get().is_selected(page_id)
                            >
                                <span>{move || page.idx() + 1}</span>
                            </button>
                        }
                    }
                />
                <div class="stepper-spacer" />
                <Button
                    label="Next Page"
                    icon="arrow_forward"
                    on:click=move |_| {
                        pages_context.update(|pages_context| pages_context.next());
                    }
                    disabled=Signal::derive(move || pages_context.get().is_last_selected())
                />
            </ButtonGroup>
        </div>
    }
}

#[component]
pub fn PagePrevNextButtons() -> impl IntoView {
    let pages_context = expect_context::<RwSignal<PagesContext>>();

    view! {
        <ButtonGroup>
            <Button
                label="Previous Page"
                icon="arrow_back"
                on:click=move |_| pages_context.update(|pages| pages.prev())
                disabled=Signal::derive(move || pages_context.get().is_first_selected())
            />
            <Button
                label="Next Page"
                icon="arrow_forward"
                on:click=move |_| {
                    pages_context.update(|pages_context| pages_context.next());
                }
                disabled=Signal::derive(move || pages_context.get().is_last_selected())
            />
        </ButtonGroup>
    }
}