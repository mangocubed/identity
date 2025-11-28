use dioxus::prelude::*;

use sdk::app::components::{Form, FormSuccessModal, H1, PageTitle, PasswordField, TextField};
use sdk::app::hooks::use_form_provider;

use crate::hooks::{use_can_register, use_current_user};
use crate::routes::Routes;
use crate::server_fns;
use crate::storage::set_session;

#[component]
pub fn LoginPage() -> Element {
    use_form_provider("login", server_fns::login);

    let mut current_user = use_current_user();
    let can_register = use_can_register();

    rsx! {
        PageTitle { "Login" }

        H1 { "Login" }

        FormSuccessModal {
            on_close: move |_| {
                current_user.restart();
            },
        }

        Form {
            on_success: move |value| {
                set_session(&serde_json::from_value(value).unwrap());
            },
            TextField {
                id: "username_or_email",
                label: "Username or email",
                name: "username_or_email",
            }

            PasswordField {
                id: "password",
                label: "Password",
                max_length: 128,
                name: "password",
            }
        }

        div { class: "login-links",
            if *can_register.read() == Some(true) {
                Link {
                    class: "btn btn-block btn-outline",
                    to: Routes::register(),
                    "I don't have an account"
                }
            }

            Link {
                class: "btn btn-block btn-outline",
                to: Routes::reset_password(),
                "I forgot my password"
            }
        }
    }
}
