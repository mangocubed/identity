use dioxus::prelude::*;
use serde_json::Value;

use sdk::components::{Form, FormSuccessModal, H1, PageTitle, PasswordField, TextField};
use sdk::hooks::use_form_provider;

use crate::hooks::use_current_user;
use crate::routes::Routes;
use crate::server_fns::{attempt_to_login, can_register_user};
use crate::set_session_token;

#[component]
pub fn LoginPage() -> Element {
    use_form_provider("login".to_owned(), attempt_to_login);

    let navigator = use_navigator();
    let mut current_user = use_current_user();
    let can_register_user = use_resource(can_register_user);

    rsx! {
        PageTitle { "Login" }

        H1 { "Login" }

        FormSuccessModal {
            on_close: move |_| {
                navigator.push(Routes::home());
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

        if can_register_user() == Some(Ok(true)) {
            div { class: "login-links",
                Link {
                    class: "btn btn-block btn-outline",
                    to: Routes::register(),
                    "I don't have an account"
                }
            }
        }
    }
}
