use dioxus::prelude::*;

use sdk::app::components::{Brand, Form, FormSuccessModal, H3, Modal, PasswordField, TextField};
use sdk::app::hooks::use_form_provider;
use sdk::app::icons::InformationCircleOutline;
use sdk::constants::{COPYRIGHT, PRIVACY_URL, TERMS_URL};
use serde_json::Value;
use uuid::Uuid;

use crate::constants::SOURCE_CODE_URL;
use crate::hooks::use_current_user;
use crate::routes::Routes;
use crate::server_fns;

#[component]
pub fn AboutModal(is_open: Signal<bool>) -> Element {
    rsx! {
        Modal { is_open, class: "gap-4 flex flex-col items-center",
            Brand { "ID" }

            div { class: "text-center text-sm opacity-75",
                p {
                    "Version: "
                    {env!("CARGO_PKG_VERSION")}
                    " ("
                    {env!("GIT_REV_SHORT")}
                    ")"
                }

                p {
                    "Built on: "
                    {env!("BUILD_DATETIME")}
                }
            }

            div {
                a { class: "link", href: TERMS_URL, target: "_blank", "Terms of Service" }

                span { class: "opacity-50", " | " }

                a { class: "link", href: PRIVACY_URL, target: "_blank", "Privacy Policy" }

                span { class: "opacity-50", " | " }

                a {
                    class: "link",
                    href: SOURCE_CODE_URL.clone(),
                    target: "_blank",
                    "Source code"
                }
            }

            div { class: "opacity-75", {COPYRIGHT} }
        }
    }
}

#[component]
pub fn ChangeEmailForm() -> Element {
    use_form_provider("update-email", server_fns::update_email);

    let mut current_user = use_current_user();

    rsx! {
        FormSuccessModal {
            on_close: move |_| {
                current_user.restart();
            },
        }

        Form {
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
        }
    }
}

#[component]
pub fn EmailConfirmationModal(is_open: Signal<bool>) -> Element {
    use_form_provider("confirm-email", server_fns::confirm_email);

    let mut current_user = use_current_user();
    let navigator = use_navigator();

    rsx! {
        FormSuccessModal {
            on_close: move |_| {
                current_user.restart();
                navigator.push(Routes::home());
            },
        }

        Modal { is_open,
            H3 { "Confirm email" }

            Form {
                on_success: move |_| {
                    is_open.set(false);
                },
                TextField {
                    id: "confirmation_code",
                    name: "code",
                    label: "Confirmation code",
                }
            }
        }
    }
}

#[component]
pub fn ResetPasswordModal(is_open: Signal<bool>, confirmation_id: Signal<Option<Uuid>>) -> Element {
    use_form_provider("reset-password", server_fns::reset_password);

    let navigator = use_navigator();

    rsx! {
        FormSuccessModal {
            on_close: move |_| {
                navigator.push(Routes::login());
            },
        }

        Modal {
            is_open,
            on_close: move |_| {
                confirmation_id.set(None);
            },
            H3 { "Change password" }

            div { role: "alert", class: "alert mt-4",
                InformationCircleOutline {}

                div { "A confirmation code has been sent to your email address." }
            }

            Form {
                on_success: move |_| {
                    is_open.set(false);
                    confirmation_id.set(None);
                },
                if let Some(confirmation_id) = confirmation_id() {
                    input {
                        r#type: "hidden",
                        name: "confirmation_id",
                        value: confirmation_id.to_string(),
                    }
                }

                TextField {
                    id: "confirmation_code",
                    name: "confirmation_code",
                    label: "Confirmation code",
                }

                PasswordField {
                    id: "password",
                    label: "Password",
                    max_length: 128,
                    name: "password",
                }
            }
        }
    }
}

#[component]
pub fn SendPasswordResetConfirmationForm(#[props(into)] on_success: Callback<Value>) -> Element {
    use_form_provider("reset-password", server_fns::send_password_reset_confirmation);

    rsx! {
        Form { on_success,
            TextField {
                id: "username_or_email",
                label: "Username or email",
                name: "username_or_email",
            }
        }
    }
}
