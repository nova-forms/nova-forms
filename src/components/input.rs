use std::{borrow::Cow, collections::HashMap, str::FromStr};

use leptos::*;
use leptos_i18n::reexports::fixed_decimal::Sign;

use crate::{Datatype, PagesContext, QueryString, Translate};

use super::PageId;


#[derive(Debug, Clone)]
pub(crate) struct InputData {
    pub(crate) page_id: PageId,
    pub(crate) label: TextProp,
    pub(crate) show_error: bool,
    pub(crate) set_filled: WriteSignal<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct InputId(Cow<'static, str>);

impl ToString for InputId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for InputId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InputId(Cow::Owned(s.to_owned())))
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct InputsContext {
    inputs: HashMap<QueryString, InputData>,
}

impl InputsContext {
    pub(crate) fn register(&mut self, qs: QueryString, data: InputData) {
        self.inputs.insert(qs, data);
    }

    pub(crate) fn set_error(&mut self, qs: &QueryString, has_error: bool) {
        self.inputs.get_mut(qs).unwrap().show_error = has_error;
    }

    pub(crate) fn has_errors(&self) -> bool {
        self.inputs.values().any(|data| data.show_error)
    }

    pub(crate) fn submit_clicked(&self) {
        for data in self.inputs.values() {
            data.set_filled.set(true);
        }
    }

    pub(crate) fn has_errors_on_page(&self, page_id: PageId) -> bool {
        self.inputs.values().any(|data| data.page_id == page_id && data.show_error)
    }

    pub(crate) fn next_clicked(&self, page_id: PageId) {
        for data in self.inputs.values().filter(|data| data.page_id == page_id) {
            logging::log!("next clicked: {}", data.label.get());
            data.set_filled.set(true);
        }
    }
}


#[component]
pub fn Input<T>(
    #[prop(into)] label: TextProp,
    #[prop(into)] bind: QueryString,
    #[prop(optional, into)] placeholder: Option<T>,
    #[prop(optional, into)] value: MaybeProp<T>,
) -> impl IntoView
where
    T: Datatype,
{
    let (qs, form_value) = bind.form_value::<T>();

    let (filled, set_filled) = create_signal(value.get_untracked()
        .or_else(|| form_value.clone().ok())
        .is_some());
    let (error, set_error) = create_signal(T::from_str(&value.get_untracked()
        .or_else(|| form_value.clone().ok())
        .map(|value| value.to_string())
        .unwrap_or_default()));
    let (pages_context, _) = expect_context::<(ReadSignal<PagesContext>, WriteSignal<PagesContext>)>();
    let (_, set_inputs_context) = expect_context::<(ReadSignal<InputsContext>, WriteSignal<InputsContext>)>();
    let show_error = Signal::derive(move || if filled.get() { error.get().is_err() } else { false });

    set_inputs_context.update(|inputs_context| inputs_context.register(qs.clone(), InputData {
        page_id: pages_context.get_untracked().pages().last().unwrap().id.clone(),
        label: label.clone(),
        show_error: show_error.get_untracked(),
        set_filled: set_filled,
    }));

    let qs_clone = qs.clone();
    let l = label.clone();
    create_effect(move |_| {
        logging::log!("label: {}, filled: {}", l.get(), filled.get());
        let qs = qs_clone.clone();
        set_inputs_context.update(move |inputs_context| {
            inputs_context.set_error(&qs, show_error.get());
        });
    });


    let input_elem = T::attributes()
        .into_iter()
        .fold(html::input(), |el, (name, value)| el.attr(name, value))
        .attr("id", qs.to_string())
        .attr("name", qs.to_string())
        .attr("value", move || {
            value.get()
                .or_else(|| form_value.clone().ok())
                .map(|value| value.to_string())
        })
        .attr("placeholder", placeholder.as_ref().map(T::to_string))
        .on(ev::input, move |ev| {
            set_error.set(T::from_str(&event_target_value(&ev)));
            set_filled.set(true);
        });

    let translate_errors = use_context::<Translate<T::Error>>();

    view! {
        <div class="field" class:error=move || show_error.get() >
            <label for=qs.to_string()>{label}</label>
            { input_elem }
            {
                move || {
                    if let (Err(err), Some(translate_errors), true) = (error.get(), translate_errors.as_ref(), show_error.get()) {
                        view! {
                            <span class="error-message">{translate_errors.clone().t(err)}</span>
                        }.into_view()
                    } else {
                        View::default()
                    }
                }
            }
        </div>
    }
}
