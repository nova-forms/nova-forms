use std::{fmt::Display, hash::Hash, str::FromStr};

use crate::{use_translation, FieldWiring, Group, QueryString};
use leptos::*;
use strum::{IntoDiscriminant, IntoEnumIterator, ParseError};

/// A component that renders radio buttons from an enum.
#[component]
pub fn Choices<T, F, IV>(
    /// The label of the input field.
    #[prop(into)] label: TextProp,
    /// The query string that binds the input field to the form data.
    #[prop(into)] bind: QueryString,
    /// Renders the choice.
    #[prop(into)] choice: F,
    /// The initial value of the input field.
    #[prop(optional, into)] value: MaybeProp<<T as IntoDiscriminant>::Discriminant>,
    /// A write signal that is updated with the parsed value of the input field.
    #[prop(optional, into)] change: Option<Callback<Result<<T as IntoDiscriminant>::Discriminant, <<T as IntoDiscriminant>::Discriminant as FromStr>::Err>, ()>>,
    /// Set a custom error message for the input field.
    #[prop(optional, into)] error: MaybeProp<TextProp>,
    #[prop(optional)] _phantom: std::marker::PhantomData<T>,
) -> impl IntoView
where
    F: Fn(<T as IntoDiscriminant>::Discriminant) -> IV + Copy + 'static,
    IV: IntoView,
    T: IntoDiscriminant + Clone + Copy + Default + Eq + Hash + Display + 'static,
    <T as IntoDiscriminant>::Discriminant: IntoEnumIterator + FromStr<Err = ParseError> + Into<&'static str> + Clone + Copy + Default + Eq + Hash + Display + 'static
{    
    let FieldWiring {
        qs,
        value,
        error,
        set_raw_value,
        ..
    } = FieldWiring::<<T as IntoDiscriminant>::Discriminant>::wire(label.clone(), bind, value, change, error);
 
    view! {
        <div
            class="field choices"
            class:error=move || error.get().is_some()
            class:ok=move || error.get().is_none()
        >   
            <fieldset>
                <legend>{label}</legend>
                <For
                    each={move || <T as IntoDiscriminant>::Discriminant::iter()}
                    key={|item| *item}
                    children={move |item| {

                        let input_elem = html::input()
                            .attr("type", "radio")
                            .attr("id", format!("{}({})", qs.to_string(), item.into()))
                            .attr("name", qs.to_string())
                            .attr("checked", move || value.get() == Ok(item))
                            .attr("value", move || item.into())
                            .on(ev::input, move |ev| {
                                set_raw_value.call(event_target_value(&ev));
                            });
                        
                        view! {
                            <label for=format!("{}({})", qs.to_string(), item.into())>
                                {input_elem}
                                <span class="custom-radio"></span>
                                <span class="custom-radio-label">{use_translation(item)}</span>
                            </label>
                            <Group bind=bind>
                                <div
                                    class="choice"
                                    class:hidden=move || value.get() != Ok(item)
                                    class:visible=move || value.get() == Ok(item)
                                >
                                    {choice(item)}
                                </div>
                            </Group>
                        }
                    }}
                />
                {move || {
                    if let Some(error) = error.get() {
                        view! { <span class="error-message">{error}</span> }
                            .into_view()
                    } else
                    if let Some(error) = error.get() {
                        view! { <span class="error-message">{error}</span> }
                            .into_view()
                    } else {
                        View::default()
                    }
                }}
            </fieldset>
            
        </div>
    }
}
