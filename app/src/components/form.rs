use leptos::either::Either;
use leptos::prelude::*;

use crate::icons::{EyeMini, EyeSlashMini};

#[component]
pub fn CountryField(
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] error: Signal<Option<String>>,
    #[prop(optional)] id: String,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] name: String,
    #[prop(into, optional)] value: Signal<String>,
) -> impl IntoView {
    view! {
        <SelectField disabled=disabled id=id label=label name=name error=error>
            <option value="" selected=move || value.get().is_empty()>
                "Select"
            </option>
            {rust_iso3166::ALL
                .iter()
                .map(|country| {
                    view! {
                        <option value=country.alpha2 selected=move || value.get() == country.alpha2>
                            {country.name}
                        </option>
                    }
                })
                .collect::<Vec<_>>()}
        </SelectField>
    }
}

#[component]
fn FormField(
    children: Children,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] error: Signal<Option<String>>,
    #[prop(into, optional)] label: String,
) -> impl IntoView {
    view! {
        <fieldset class="fieldset" disabled=disabled>
            <legend class="fieldset-legend empty:hidden">{label}</legend>

            {children()}

            <div class="label text-error empty:hidden">{error}</div>
        </fieldset>
    }
}

#[component]
pub fn PasswordField(
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] error: Signal<Option<String>>,
    #[prop(optional)] id: String,
    #[prop(into, optional)] label: String,
    #[prop(default = 255)] max_length: u16,
    #[prop(into, optional)] name: String,
    #[prop(into, optional)] readonly: Signal<bool>,
) -> impl IntoView {
    let input_type = RwSignal::new("password");

    view! {
        <FormField disabled=disabled error=error label=label>
            <div class="input flex items-center gap-2 pr-0" class:input-error=move || error.read().is_some()>
                <input
                    class="grow"
                    disabled=disabled
                    id=id.clone()
                    maxlength=max_length
                    name=name
                    readonly=readonly
                    type=input_type
                />

                <button
                    class="btn btn-ghost btn-sm"
                    disabled=disabled
                    on:click=move |event| {
                        event.prevent_default();
                        if *readonly.read_untracked() {
                            return;
                        }
                        input_type
                            .update(|input_type| {
                                *input_type = if *input_type == "password" { "text" } else { "password" };
                            });
                    }
                >
                    {move || {
                        if input_type.read() == "password" {
                            Either::Left(view! { <EyeSlashMini /> })
                        } else {
                            Either::Right(view! { <EyeMini /> })
                        }
                    }}
                </button>
            </div>
        </FormField>
    }
}

#[component]
pub fn SelectField(
    children: Children,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] error: Signal<Option<String>>,
    #[prop(optional)] id: String,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] name: String,
) -> impl IntoView {
    view! {
        <FormField disabled=disabled error=error label=label>
            <select class="select" class:select-error=move || error.read().is_some() disabled=disabled id=id name=name>
                {children()}
            </select>
        </FormField>
    }
}

#[component]
pub fn SubmitButton(
    #[prop(default = "Submit".to_owned())] label: String,
    #[prop(optional, into)] is_pending: Signal<bool>,
) -> impl IntoView {
    view! {
        <button class="btn-submit" type="submit" disabled=is_pending>
            {move || {
                if is_pending.get() {
                    Either::Left(view! { <span class="loading loading-spinner" /> })
                } else {
                    Either::Right(label.clone())
                }
            }}
        </button>
    }
}

#[component]
pub fn TextField(
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] error: Signal<Option<String>>,
    #[prop(optional)] id: String,
    #[prop(default = "text".to_owned(), into)] input_type: String,
    #[prop(into, optional)] label: String,
    #[prop(default = 255)] max_length: u16,
    #[prop(into, optional)] name: String,
    #[prop(into, optional)] readonly: Signal<bool>,
    #[prop(into, optional)] value: Signal<String>,
) -> impl IntoView {
    view! {
        <FormField disabled=disabled error=error label=label>
            <input
                class="input"
                class:input-error=move || error.read().is_some()
                disabled=disabled
                id=id
                maxlength=max_length
                name=name
                readonly=readonly
                r#type=input_type
                value=value
            />
        </FormField>
    }
}
