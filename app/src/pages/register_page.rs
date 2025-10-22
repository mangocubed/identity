use dioxus::prelude::*;

use sdk::app::components::{Form, FormSuccessModal, H1, PageTitle, PasswordField, SelectField, TextField};
use sdk::app::hooks::use_form_provider;
use sdk::app::icons::InformationCircleOutline;
use sdk::constants::{PRIVACY_URL, TERMS_URL};
use serde_json::Value;

use crate::hooks::{use_can_register, use_current_user};
use crate::local_data::set_session_token;
use crate::requests;
use crate::routes::Routes;

#[component]
pub fn RegisterPage() -> Element {
    use_form_provider("register", requests::register);

    let navigator = use_navigator();
    let mut current_user = use_current_user();
    let can_register = use_can_register();

    use_effect(move || {
        if *can_register.read() == Some(false) {
            navigator.push(Routes::login());
        }
    });

    rsx! {
        PageTitle { "Register" }

        H1 { "Register" }

        FormSuccessModal {
            on_close: move |_| {
                current_user.restart();
            },
        }

        Form {
            on_success: move |value| {
                if let Value::String(token) = value {
                    set_session_token(&token);
                }
            },
            TextField {
                id: "username",
                label: "Username",
                max_length: 16,
                name: "username",
            }

            TextField {
                id: "email",
                input_type: "email",
                label: "Email",
                name: "email",
            }

            PasswordField {
                id: "password",
                label: "Password",
                max_length: 128,
                name: "password",
            }

            TextField { id: "full_name", label: "Full name", name: "full_name" }

            TextField {
                id: "birthdate",
                label: "Birthdate",
                name: "birthdate",
                input_type: "date",
            }

            SelectField {
                id: "country_alpha2",
                label: "Country",
                name: "country_alpha2",
                option { "Select" }
                for country in rust_iso3166::ALL {
                    option { value: country.alpha2, {country.name} }
                }
            }

            div { class: "alert mt-4 mb-2",
                InformationCircleOutline {}

                p {
                    "By submitting this form, you are declaring that you accept our "

                    a { class: "link", href: TERMS_URL, target: "_blank", "Terms of Service" }

                    " and "

                    a { class: "link", href: PRIVACY_URL, target: "_blank", "Privacy Policy" }

                    "."
                }
            }
        }

        div { class: "login-links",
            Link { class: "btn btn-block btn-outline", to: Routes::login(), "Back to login" }
        }
    }
}
