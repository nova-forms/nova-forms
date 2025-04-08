use std::{fmt::Debug, marker::PhantomData, str::FromStr};

use crate::{BaseGroupContext, GroupContext, QueryString};
use leptos::*;

mod context {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct ChoicesContext<T: Clone + Eq + 'static> {
        selected: Signal<T>,
    }

    impl<T> ChoicesContext<T>
    where
        T: Clone + Eq + 'static
    {
        pub fn new<F>(f: F) -> Self
        where
            F: Fn() -> T + 'static,
        {
            Self {
                selected: Signal::derive(f)
            }
        }

        pub fn selected(&self, discriminant: T) -> Signal<bool> {
            let selected = self.selected;
            Signal::derive(move || selected.get() == discriminant)
        }
    }
}

pub(crate) use context::ChoicesContext;

/// A component that renders radio buttons from an enum.
#[component]
pub fn Choices<T>(
    /// Tag to determine the type.
    #[prop(into)] tag: QueryString,
    #[prop(optional)] _phantom: PhantomData<T>,
    /// Child components.
    children: Children,
) -> impl IntoView
where
    T: Clone + Eq + Default + Debug + FromStr + 'static,
    T::Err: Clone + Debug + 'static,
{   
    let parent_group = expect_context::<GroupContext>();
    let base_group = expect_context::<BaseGroupContext>();
    let value: Signal<Result<T, <T as FromStr>::Err>> = Signal::derive(move || base_group
        .get(parent_group.qs().join(tag))
        .get()
        .expect(&format!("tag {} not found", base_group.qs().join(tag)))
        .as_input()
        .expect("expected input, got group").value::<T>()
        .get());
    
    provide_context(ChoicesContext::new(move || value.get().unwrap_or_default()));

    let children = children();

    view! {
        <div class="choices">
            {children}
        </div>
    }
}

#[component]
pub fn Choice<T>(
    /// The label of the input field.
    #[prop(into)] discriminant: T,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView
where
    T: Copy + Eq + 'static
{    
    let context = expect_context::<ChoicesContext<T>>();

    view! {
        <div class=move || {
            if context.selected(discriminant).get() { "choice selected" } else { "choice hidden" }
        }>
            {if let Some(children) = children { children().into_view() } else { View::default() }}
        </div>
    }
}
