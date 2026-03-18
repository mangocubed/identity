use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::{Alert, AlertType, PasswordField, SubmitButton};
use crate::hooks::use_toast;
use crate::server_fns::{ActionResultExt, UpdatePassword};

use super::AuthenticatedPage;

#[component]
pub fn ChangePasswordPage() -> impl IntoView {
    let navigate = use_navigate();
    let mut toast = use_toast();
    let action = ServerAction::<UpdatePassword>::new();
    let action_value = action.value();
    let error_current_password = Memo::new(move |_| action_value.read().get_param_error("current_password"));
    let error_new_password = Memo::new(move |_| action_value.read().get_param_error("new_password"));

    Effect::watch(
        move || action_value.get(),
        move |action_value, _, _| {
            if action_value.is_success() {
                toast.push_alert(AlertType::Success, "Password changed successfully");
                navigate("/", Default::default());
            }
        },
        false,
    );

    view! {
        <AuthenticatedPage title="Change Password">
            <ActionForm action=action attr:class="form" attr:autocomplete="off" attr:novalidate="true">
                <Show when=move || action_value.read().has_errors()>
                    <Alert alert_type=AlertType::Error>"Failed to update password"</Alert>
                </Show>

                <PasswordField
                    disabled=action.pending()
                    label="Current password"
                    name="current_password"
                    error=error_current_password
                />

                <PasswordField
                    disabled=action.pending()
                    label="New password"
                    name="new_password"
                    error=error_new_password
                />

                <SubmitButton is_pending=action.pending() />
            </ActionForm>
        </AuthenticatedPage>
    }
}
