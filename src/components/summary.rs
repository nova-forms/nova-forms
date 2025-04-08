use leptos::*;

use crate::{BaseGroupContext, GroupContext, InputContext, Node};

#[component]
pub fn Summary() -> impl IntoView {
    let group = expect_context::<BaseGroupContext>();

    view! {
        <div class="summary">
            <SummaryGroup group=group.to_group_context() />
        </div>
    }
}

#[component]
fn SummaryGroup(
    group: GroupContext,
) -> impl IntoView {
    view! {
        <div class="summary-group">
            <span class="summary-group-label">{group.label()}</span>
            <div class="summary-group-content">
                <For
                    each={move || group.nodes()}
                    key={move |node| node.qs()}
                    children={move |node| {
                        view! {
                            <div class="summary-group-node">
                                {match node {
                                    Node::Group(group) => view! {
                                        <SummaryGroup group=group />
                                    },
                                    Node::Input(input) => view! {
                                        <SummaryInput input=input />
                                    }
                                }}
                            </div>
                        }
                    }}
                />
            </div>
        </div>
    }
}

#[component]
fn SummaryInput(
    input: InputContext,
) -> impl IntoView {
    view! {
        <div class="summary-input">
            <span class="summary-input-label">{input.label()}</span>
            <span class="summary-input-value">{input.raw_value()}</span>
        </div>
    }
}
