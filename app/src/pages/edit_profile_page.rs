use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::{Alert, AlertType, CountryField, CurrentUser, SubmitButton, TextField};
use crate::hooks::use_toast;
use crate::pages::AuthenticatedPage;
use crate::server_fns::{ActionResultExt, UpdateProfile};

#[component]
pub fn EditProfilePage() -> impl IntoView {
    let navigate = use_navigate();
    let mut toast = use_toast();
    let action = ServerAction::<UpdateProfile>::new();
    let action_value = action.value();
    let error_display_name = Memo::new(move |_| action_value.read().get_param_error("display_name"));
    let error_full_name = Memo::new(move |_| action_value.read().get_param_error("full_name"));
    let error_birthdate = Memo::new(move |_| action_value.read().get_param_error("birthdate"));
    let error_country_code = Memo::new(move |_| action_value.read().get_param_error("country_code"));

    Effect::watch(
        move || action_value.get(),
        move |action_value, _, _| {
            if action_value.is_success() {
                toast.push_alert(AlertType::Success, "Profile updated successfully");
                navigate("/", Default::default());
            }
        },
        false,
    );

    view! {
        <AuthenticatedPage title="Edit Profile">
            <ActionForm action=action attr:class="form" attr:autocomplete="off" attr:novalidate="true">
                <Show when=move || action_value.read().has_errors()>
                    <Alert alert_type=AlertType::Error>"Failed to update profile"</Alert>
                </Show>

                <CurrentUser children=move |result| {
                    result
                        .map(|user| {
                            view! {
                                <TextField
                                    disabled=action.pending()
                                    label="Display name"
                                    name="display_name"
                                    value=user.display_name
                                    error=error_display_name
                                />

                                <TextField
                                    disabled=action.pending()
                                    label="Full name"
                                    name="full_name"
                                    value=user.full_name
                                    error=error_full_name
                                />

                                <TextField
                                    disabled=action.pending()
                                    label="Birthdate"
                                    input_type="date"
                                    name="birthdate"
                                    value=user.birthdate.to_string()
                                    error=error_birthdate
                                />

                                <CountryField
                                    disabled=action.pending()
                                    label="Country"
                                    name="country_code"
                                    value=user.country_code
                                    error=error_country_code
                                />

                                <SubmitButton is_pending=action.pending() />
                            }
                        })
                } />
            </ActionForm>
        </AuthenticatedPage>
    }
}
