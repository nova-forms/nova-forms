use std::{fmt::Debug, str::FromStr};

use leptos::*;
use leptos_i18n::{I18nContext, Locale, LocaleKeys};

use crate::{start_preview, stop_preview, use_translation, ButtonGroup, Button, SelectButton, NovaFormContext, PagesContext, Translation};

#[component]
pub fn Toolbar(
    children: Children,
) -> impl IntoView {
    view! { <aside id="toolbar"><ButtonGroup>{children()}</ButtonGroup></aside> }
}

#[component]
pub fn ToolbarSubmitButton() -> impl IntoView {
    let nova_form_context = expect_context::<NovaFormContext>();

    view! {
        <Button
            button_type="submit"
            label=use_translation(Translation::Submit)
            icon="send"
            form=nova_form_context.form_id()
            disabled=cfg!(feature = "csr")
        />

    }
}

#[component]
pub fn ToolbarPreviewButton() -> impl IntoView {
    let nova_form_context = expect_context::<NovaFormContext>();

    view! {
        {move || {
            if nova_form_context.is_preview_mode() {
                view! {
                    <Button
                        label=use_translation(Translation::Edit)
                        icon="edit"
                        on:click=move |_| {
                            stop_preview(nova_form_context.form_id());
                            nova_form_context.edit_mode();
                        }
                    />
                }
            } else {
                view! {
                    <Button
                        label=use_translation(Translation::Preview)
                        icon="visibility"
                        on:click=move |_| {
                            nova_form_context.preview_mode();
                            start_preview(nova_form_context.form_id());
                        }
                    />
                }
            }
        }}
    }
}

#[component]
pub fn ToolbarLocaleSelect<L, K>(
    i18n: I18nContext<L, K>,
) -> impl IntoView
where
    L: Locale + 'static,
    <L as FromStr>::Err: Debug,
    K: LocaleKeys<Locale = L> + 'static,
{
    let nova_form_context = expect_context::<NovaFormContext>();

    let locales = L::get_all()
        .iter()
        .map(|locale| {
            let id = &locale.as_icu_locale().id;
            let language_str = match id.language.as_str() {
                "en" => "English",
                "de" => "Deutsch",
                "fr" => "Français",
                "it" => "Italiano",
                "es" => "Español",
                other => other,
            };
            let region = id.region.as_ref();
            let region_str = match region {
                Some(region) => match region.as_str() {
                    "US" => "🇺🇸",
                    "GB" => "🇬🇧",
                    "DE" => "🇩🇪",
                    "CH" => "🇨🇭",
                    "FR" => "🇫🇷",
                    "IT" => "🇮🇹",
                    "ES" => "🇪🇸",
                    other => other,
                },
                None => match id.language.as_str() {
                    "en" => "🇺🇸",
                    "de" => "🇩🇪",
                    "fr" => "🇫🇷",
                    "it" => "🇮🇹",
                    "es" => "🇪🇸",
                    _ => "",
                },
            };
            (
                *locale,
                if region_str.is_empty() {
                    format!("{}", language_str)
                } else {
                    format!("{} {}", region_str, language_str)
                }
                .into(),
            )
        })
        .collect::<Vec<_>>();

    view! {
        <SelectButton
            id="language"
            label=use_translation(Translation::Language)
            icon="translate"
            values=locales.clone()
            value=move || i18n.get_locale()
            on_change=move |locale| {
                i18n.set_locale(locale);
                start_preview(nova_form_context.form_id());
            }
        />
    }
}


#[component]
pub fn ToolbarPageSelect(
) -> impl IntoView {
    let pages_context = expect_context::<RwSignal<PagesContext>>();


    let pages = pages_context
        .get_untracked()
        .pages()
        .iter()
        .map(|page| (page.id(), page.label().clone()))
        .collect::<Vec<_>>();

    view! {
        <Show when=move || { pages_context.get().len() > 1 }>
            <SelectButton
                id="menu"
                label=use_translation(Translation::Menu)
                icon="menu"
                values=pages.clone()
                value=move || pages_context.get().selected().expect("page index out of bounds")
                on_change=move |tab_id| {
                    pages_context.update(|pages_context| pages_context.select(tab_id))
                }
            />
        </Show>
    }
    
}