use dioxus::prelude::*;

use sdk::app::components::{H1, H2, PageTitle};
use sdk::app::hooks::use_action_with_spinner;

use crate::components::{ChangeEmailForm, EmailConfirmationModal};
use crate::hooks::use_current_user;
use crate::server_fns;

#[component]
pub fn EmailPage() -> Element {
    let current_user = use_current_user();
    let mut send_email_confirmation = use_action_with_spinner("send-email-confirmation", move |()| {
        server_fns::send_email_confirmation()
    });
    let mut show_confirmation_modal = use_signal(|| false);

    use_effect(move || {
        if send_email_confirmation.value().is_some_and(|result| result.is_ok()) {
            show_confirmation_modal.set(true);
            send_email_confirmation.reset();
        }
    });

    rsx! {
        PageTitle { "Email" }

        H1 { "Email" }

        div { class: "max-w-[640px] w-full mx-auto",
            section { class: "my-6",
                H2 { "Current email" }

                div { class: "flex justify-between",
                    if let Some(Ok(user)) = &*current_user.read() {
                        div { class: "font-bold", {user.email.clone()} }

                        if user.email_is_confirmed {
                            div { class: "badge badge-outline badge-accept", "Confirmed" }
                        } else {
                            button {
                                class: "btn btn-sm btn-outline",
                                onclick: move |_| {
                                    send_email_confirmation.call(());
                                },
                                if send_email_confirmation.pending() {
                                    span { class: "loading loading-spinner" }
                                } else {
                                    "Send confirmation code"
                                }
                            }

                            EmailConfirmationModal { is_open: show_confirmation_modal }
                        }
                    }
                }
            }

            section { class: "my-6",
                H2 { "Change email" }

                ChangeEmailForm {}
            }
        }
    }
}
