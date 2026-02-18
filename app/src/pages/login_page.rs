use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;
use url::form_urlencoded;

use crate::components::{Alert, AlertType, PasswordField, SubmitButton, TextField};
use crate::hooks::{use_current_user_resource, use_redirect_to, use_toast};
use crate::server_fns::{ActionResultExt, CreateSession};

use super::GuestPage;

#[component]
pub fn LoginPage() -> impl IntoView {
    let navigate = use_navigate();
    let current_user_resource = use_current_user_resource();
    let mut toast = use_toast();
    let redirect_to = use_redirect_to();
    let action = ServerAction::<CreateSession>::new();
    let action_value = action.value();
    let error_username_or_email = Memo::new(move |_| action_value.read().get_param_error("username_or_email"));
    let error_password = Memo::new(move |_| action_value.read().get_param_error("password"));

    Effect::watch(
        move || action_value.get(),
        move |action_value, _, _| {
            if action_value.is_success() {
                current_user_resource.refetch();
                toast.push_alert(AlertType::Success, "Session started successfully");
                navigate(&redirect_to.get_untracked(), Default::default());
            }
        },
        false,
    );

    view! {
        <GuestPage title="Login">
            <ActionForm
                action=action
                attr:class="form"
                attr:autocomplete="off"
                attr:novalidate="true"
            >
                <Show when=move || action_value.read().has_errors()>
                    <Alert alert_type=AlertType::Error>"Failed to authenticate user"</Alert>
                </Show>

                <TextField
                    disabled=action.pending()
                    label="Username or email"
                    name="username_or_email"
                    error=error_username_or_email
                />

                <PasswordField
                    disabled=action.pending()
                    label="Password"
                    name="password"
                    error=error_password
                />

                <SubmitButton is_pending=action.pending() />
            </ActionForm>
            <div class="login-links">
                <A
                    attr:class="btn btn-block btn-outline"
                    href=move || {
                        format!(
                            "/register?redirect_to={}",
                            form_urlencoded::byte_serialize(redirect_to.get().as_bytes())
                                .collect::<String>(),
                        )
                    }
                >
                    "I don't have an account"
                </A>
            </div>
        </GuestPage>
    }
}
