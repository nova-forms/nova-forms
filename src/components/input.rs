use std::{borrow::Cow, collections::HashMap, str::FromStr};

use leptos::*;

use crate::{Datatype, PagesContext, QueryString, Translate};

use super::PageId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum TriggerValidation {
    None,
    All,
    Page(PageId),
}

#[derive(Debug, Clone)]
pub(crate) struct InputData {
    pub(crate) page_id: PageId,
    pub(crate) label: TextProp,
    pub(crate) has_error: bool,
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

#[derive(Debug, Clone)]
pub(crate) struct InputsContext {
    inputs: HashMap<QueryString, InputData>,
    pub trigger_validation: ReadSignal<TriggerValidation>,
}

impl InputsContext {
    pub fn new(trigger_validation: ReadSignal<TriggerValidation>) -> Self {
        Self {
            inputs: HashMap::new(),
            trigger_validation,
        }
    }

    pub fn register(&mut self, qs: QueryString, data: InputData) {
        self.inputs.insert(qs, data);
    }

    pub fn set_error(&mut self, qs: &QueryString, has_error: bool) {
        self.inputs.get_mut(qs).unwrap().has_error = has_error;
    }

    pub fn has_errors(&self) -> bool {
        self.inputs.values().any(|data| data.has_error)
    }

    pub fn has_errors_on_page(&self, page_id: PageId) -> bool {
        self.inputs.values().any(|data| data.page_id == page_id && data.has_error)
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

    let (show_error, set_show_error) = create_signal(value.get_untracked()
        .or_else(|| form_value.clone().ok())
        .is_some());

    let (value, set_value) = create_signal(value.get_untracked()
        .or_else(|| form_value.clone().ok())
        .map(|v| v.to_string())
        .unwrap_or_default());

 
    let (pages_context, _) = expect_context::<(ReadSignal<PagesContext>, WriteSignal<PagesContext>)>();
    let (inputs_context, set_inputs_context) = expect_context::<(ReadSignal<InputsContext>, WriteSignal<InputsContext>)>();

    let page_id = pages_context.get_untracked().pages().last().unwrap().id.clone();
    
    let parsed_value = Signal::derive(move || {
        T::from_str(&value.get())
    });
    
    set_inputs_context.update(|inputs_context| inputs_context.register(qs.clone(), InputData {
        page_id: page_id.clone(),
        label: label.clone(),
        has_error: false,
    }));

    let page_id_clone: PageId = page_id.clone();
    create_effect(move |_| {
        let do_trigger = match inputs_context.get().trigger_validation.get() {
            TriggerValidation::None => false,
            TriggerValidation::All => true,
            TriggerValidation::Page(trigger_page_id) => trigger_page_id == page_id_clone,
        };

        logging::log!("trigger_validation: {}", do_trigger);

        if do_trigger {
            set_value.update(|value| {})
        }
    });



    let qs_clone = qs.clone();
    create_effect(move |_| {
        let qs = qs_clone.clone();
        set_inputs_context.update(move |inputs_context| {
            inputs_context.set_error(&qs, parsed_value.get().is_err());
        });
    });


    let input_elem = T::attributes()
        .into_iter()
        .fold(html::input(), |el, (name, value)| el.attr(name, value))
        .attr("id", qs.to_string())
        .attr("name", qs.to_string())
        .attr("value", move || {
            value.get()
        })
        .attr("placeholder", placeholder.as_ref().map(T::to_string))
        .on(ev::input, move |ev| {
            set_value.set(event_target_value(&ev));
            set_show_error.set(true);
        });

    let translate_errors = use_context::<Translate<T::Error>>();

    view! {
        <div class="field" class:error=move || parsed_value.get().is_err() && show_error.get() >
            <label for=qs.to_string()>{label}</label>
            { input_elem }
            {
                move || {
                    if let (Err(err), Some(translate_errors), true) = (parsed_value.get(), translate_errors.as_ref(), show_error.get()) {
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
