use dioxus::prelude::*;

use sdk::app::components::{H1, PageTitle};
use serde_json::Value;
use uuid::Uuid;

use crate::components::{ResetPasswordModal, SendPasswordResetConfirmationForm};
use crate::routes::Routes;

#[component]
pub fn ResetPasswordPage() -> Element {
    let mut show_reset_password_modal = use_signal(|| false);
    let mut confirmation_id = use_signal(|| None);

    rsx! {
        PageTitle { "Reset password" }

        H1 { "Reset password" }

        SendPasswordResetConfirmationForm {
            on_success: move |value| {
                if let Value::String(id) = value {
                    confirmation_id.set(Uuid::try_parse(&id).ok());
                    show_reset_password_modal.set(true);
                }
            },
        }

        ResetPasswordModal { confirmation_id, is_open: show_reset_password_modal }

        div { class: "login-links",
            Link { class: "btn btn-block btn-outline", to: Routes::login(), "Back to login" }
        }
    }
}
