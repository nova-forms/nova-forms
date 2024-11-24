use crate::{FormData, QueryString};
use leptos::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct InputContext(RwSignal<InputData>);

#[derive(Debug, Clone)]
struct InputData {
    label: TextProp,
    error: bool,
    disabled: bool,
    validate: ReadSignal<bool>,
    set_validate: WriteSignal<bool>,
    qs: QueryString,
}

impl InputContext {
    pub fn new(bind: QueryString, label: TextProp) -> Self {
        let parent_group = use_context::<GroupContext>();

        let (validate, set_validate) = create_signal(false);
        let input = InputContext(RwSignal::new(InputData {
            label,
            error: false,
            disabled: false,
            validate,
            set_validate,
            qs: parent_group.map(|parent_group| parent_group.qs()).unwrap_or_default().join(bind),
        }));

        if let Some(parent_group) = parent_group {
            parent_group.register_input(&bind, input);
            on_cleanup(move || {
                parent_group.deregister(&bind);
            });    
        }

        input
    }

    pub fn set_error(&self, has_error: bool) {
        self.0.update(|input| {
            input.error = has_error;
        });
    }

    pub fn error(&self) -> bool {
        self.0.get().error
    }

    pub fn set_disabled(&self, disabled: bool) {
        self.0.update(|input| {
            input.disabled = disabled;
        });
    }

    pub fn disabled(&self) -> bool {
        self.0.get().disabled
    }

    pub fn validate(&self) {
        self.0.get().set_validate.set(true);
    }

    pub fn validate_signal(&self) -> Signal<bool> {
        self.0.get_untracked().validate.into()
    }

    pub fn label(&self) -> TextProp {
        self.0.get_untracked().label.clone()
    }

    pub fn qs(&self) -> QueryString {
        self.0.get_untracked().qs
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GroupContext(RwSignal<GroupData>);

#[derive(Debug, Clone)]
struct GroupData {
    inputs: HashMap<QueryString, Node>,
    qs: QueryString,
}

impl GroupContext {
    pub fn new(bind: QueryString) -> Self {
        let parent_group = use_context::<GroupContext>();

        let group = GroupContext(RwSignal::new(GroupData {
            inputs: HashMap::new(),
            qs: parent_group.map(|parent_group| parent_group.qs()).unwrap_or_default().join(bind),
        }));

        if let Some(parent_group) = parent_group {
            parent_group.register_group(&bind, group);
            on_cleanup(move || {
                parent_group.deregister(&bind);
            });    
        }

        group
    }

    pub fn qs(&self) -> QueryString {
        self.0.get_untracked().qs
    }

    pub fn form_data(&self) -> FormData {
        FormData::with_context(&self.qs())
    }

    fn register_input(&self, qs: &QueryString, input: InputContext) {
        self.0.update(|data| {
            data.inputs.insert(*qs, Node::Input(input));
        });
    }

    fn register_group(&self, qs: &QueryString, group: GroupContext) {
        self.0.update(|data| {
            data.inputs.insert(*qs, Node::Group(group));
        });
    }

    pub fn deregister(&self, qs: &QueryString) {
        self.0.update(|data| {
            data.inputs.remove(qs);
        });
    }

    pub fn set_disabled(&self, disabled: bool) {
        self.0.update(|data| {
            for node in data.inputs.values_mut() {
                match node {
                    Node::Input(input) => input.set_disabled(disabled),
                    Node::Group(group) => group.set_disabled(disabled),
                }
            }
        });
    }

    pub fn error(&self) -> bool {
        self.0.get().inputs.values().any(|node| {
            match node {
                Node::Input(input) => input.error(),
                Node::Group(group) => group.error(),
            }
        })
    }

    pub fn validate(&self) {
        self.0.update(|data| {
            for node in data.inputs.values_mut() {
                match node {
                    Node::Input(input) => {
                        input.validate();
                    }
                    Node::Group(group) => {
                        group.validate();
                    }
                }
            }
        });
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Input(InputContext),
    Group(GroupContext),
}