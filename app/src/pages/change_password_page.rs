use dioxus::prelude::*;

use sdk::app::components::{Form, FormSuccessModal, H1, PageTitle, PasswordField};
use sdk::app::hooks::use_form_provider;

use crate::routes::Routes;
use crate::server_fns;

#[component]
pub fn ChangePasswordPage() -> Element {
    use_form_provider("change-password", server_fns::change_password);

    let navigator = use_navigator();

    rsx! {
        PageTitle { "Change Password" }

        H1 { "Change Password" }

        FormSuccessModal {
            on_close: move |_| {
                navigator.push(Routes::home());
            },
        }

        Form {
            PasswordField {
                id: "current_password",
                label: "Current password",
                max_length: 256,
                name: "current_password",
            }

            PasswordField {
                id: "new_password",
                label: "New password",
                max_length: 128,
                name: "new_password",
            }
        }
    }
}
