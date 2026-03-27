use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;
use url::form_urlencoded;
use uuid::Uuid;

use crate::components::{Alert, AlertType, Modal, PasswordField, SubmitButton, TextField};
use crate::hooks::{use_redirect_to, use_toast};
use crate::server_fns::{ActionResultExt, ResetPassword, SendPasswordResetConfirmation};

use super::GuestPage;

#[component]
pub fn ResetPasswordPage() -> impl IntoView {
    let redirect_to = use_redirect_to();
    let action = ServerAction::<SendPasswordResetConfirmation>::new();
    let action_value = action.value();
    let error_username_or_email = Memo::new(move |_| action_value.read().get_param_error("username_or_email"));
    let confirmation_id = Memo::new(move |_| action_value.read().as_ref().and_then(|result| result.clone().ok()));
    let show_modal = RwSignal::new(false);

    Effect::watch(
        move || action_value.get(),
        move |action_value, _, _| {
            if action_value.is_success() {
                show_modal.set(true);
            }
        },
        false,
    );

    view! {
        <GuestPage title="Reset password">
            <ActionForm action=action attr:class="form" attr:autocomplete="off" attr:novalidate="true">
                <Show when=move || action_value.read().has_errors()>
                    <Alert alert_type=AlertType::Error>"Failed to send password reset confirmation"</Alert>
                </Show>

                <TextField
                    disabled=action.pending()
                    label="Username or email"
                    name="username_or_email"
                    error=error_username_or_email
                />

                <SubmitButton is_pending=action.pending() />
            </ActionForm>

            <ResetPasswordModal confirmation_id=confirmation_id is_open=show_modal />

            <div class="login-links">
                <A
                    attr:class="btn btn-block btn-outline"
                    href=move || {
                        format!(
                            "/login?redirect_to={}",
                            form_urlencoded::byte_serialize(redirect_to.get().as_bytes()).collect::<String>(),
                        )
                    }
                >
                    "Back to login"
                </A>
            </div>
        </GuestPage>
    }
}

#[component]
fn ResetPasswordModal(confirmation_id: Memo<Option<Uuid>>, is_open: RwSignal<bool>) -> impl IntoView {
    let redirect_to = use_redirect_to();
    let navigate = use_navigate();
    let mut toast = use_toast();
    let action = ServerAction::<ResetPassword>::new();
    let action_value = action.value();
    let error_confirmation_code = Memo::new(move |_| action_value.read().get_param_error("confirmation_code"));
    let error_new_password = Memo::new(move |_| action_value.read().get_param_error("new_password"));

    Effect::watch(
        move || action_value.get(),
        move |action_value, _, _| {
            if action_value.is_success() {
                toast.push_alert(AlertType::Success, "Email confirmed successfully");
                navigate(
                    &format!(
                        "/login?redirect_to={}",
                        form_urlencoded::byte_serialize(redirect_to.get_untracked().as_bytes()).collect::<String>(),
                    ),
                    Default::default(),
                );
            }
        },
        false,
    );

    view! {
        <Modal is_closable=false is_open=is_open>
            <h3 class="h3">"Change password"</h3>

            <ActionForm action=action attr:class="form" attr:autocomplete="off" attr:novalidate="true">
                <Show when=move || action_value.read().has_errors()>
                    <Alert alert_type=AlertType::Error>"Failed to update password"</Alert>
                </Show>

                <input
                    type="hidden"
                    name="confirmation_id"
                    value=move || confirmation_id.get().as_ref().map(|id| id.to_string())
                />

                <TextField
                    disabled=action.pending()
                    label="Confirmation code"
                    name="confirmation_code"
                    error=error_confirmation_code
                />

                <PasswordField
                    disabled=action.pending()
                    label="New password"
                    name="new_password"
                    error=error_new_password
                />

                <SubmitButton is_pending=action.pending() />
            </ActionForm>
        </Modal>
    }
}
