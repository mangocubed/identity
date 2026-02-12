use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::{Alert, AlertType, PasswordField, SelectField, SubmitButton, TextField};
use crate::hooks::{use_redirect_to_cookie, use_toast};
use crate::pages::GuestPage;
use crate::server_fns::{ActionResultExt, CreateUser};

#[component]
pub fn RegisterPage() -> impl IntoView {
    let navigate = use_navigate();
    let mut toast = use_toast();
    let action = ServerAction::<CreateUser>::new();
    let action_value = action.value();
    let error_username = Memo::new(move |_| action_value.read().get_field_error("username"));
    let error_email = Memo::new(move |_| action_value.read().get_field_error("email"));
    let error_password = Memo::new(move |_| action_value.read().get_field_error("password"));
    let error_full_name = Memo::new(move |_| action_value.read().get_field_error("full_name"));
    let error_birthdate = Memo::new(move |_| action_value.read().get_field_error("birthdate"));
    let error_country_code = Memo::new(move |_| action_value.read().get_field_error("country_code"));
    let (get_redirect_to, _) = use_redirect_to_cookie();

    Effect::new(move |_| {
        if action_value.read().is_success() {
            toast.push_alert(AlertType::Success, "User created successfully");
            navigate(
                &get_redirect_to.with(|value| value.clone().unwrap_or("/".to_owned())),
                Default::default(),
            );
        }
    });

    view! {
        <GuestPage title="Register">
            <ActionForm action=action attr:class="form">
                <Show when=move || action_value.read().has_errors()>
                    <Alert alert_type=AlertType::Error>"Failed to create user"</Alert>
                </Show>

                <TextField disabled=action.pending() label="Username" name="username" error=error_username />

                <TextField disabled=action.pending() label="Email" input_type="email" name="email" error=error_email />

                <PasswordField disabled=action.pending() label="Password" name="password" error=error_password />

                <TextField disabled=action.pending() label="Full name" name="full_name" error=error_full_name />

                <TextField
                    disabled=action.pending()
                    label="Birthdate"
                    input_type="date"
                    name="birthdate"
                    error=error_birthdate
                />

                <SelectField disabled=action.pending() label="Country" name="country_code" error=error_country_code>
                    <option value="">"Select"</option>
                </SelectField>

                <Alert>
                    "By submitting this form, you are declaring that you accept our "
                    <a class="link" href="https://mango3.app/terms" target="_blank">
                        "Terms of Service"
                    </a> " and " <a class="link" href="https://mango3.app/privacy" target="_blank">
                        "Privacy Policy"
                    </a> "."
                </Alert>

                <SubmitButton is_pending=action.pending() />
            </ActionForm>
        </GuestPage>
    }
}
