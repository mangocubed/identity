use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::{Alert, AlertType, CurrentUser, Modal, PasswordField, SubmitButton, TextField};
use crate::hooks::{use_current_user_resource, use_toast};
use crate::server_fns::{ActionResultExt, ConfirmEmail, SendEmailConfirmation, UpdateEmail};

use super::AuthenticatedPage;

#[component]
pub fn EditEmailPage() -> impl IntoView {
    let send_confirmation_action = ServerAction::<SendEmailConfirmation>::new();
    let send_confirmation_action_value = send_confirmation_action.value();
    let show_confirmation_modal = RwSignal::new(false);

    Effect::watch(
        move || send_confirmation_action_value.get(),
        move |action_value, _, _| {
            if action_value.is_success() {
                show_confirmation_modal.set(true);
            }
        },
        false,
    );

    view! {
        <AuthenticatedPage title="Edit email">
            <section class="my-6">
                <h2 class="h2">"Current email"</h2>

                <div class="flex justify-between">
                    <CurrentUser children=move |user| {
                        user.map(|user| {
                            view! {
                                <div class="font-bold">{user.email}</div>
                                {if user.email_is_confirmed {
                                    Either::Left(
                                        view! { <div class="badge badge-outline badge-accept">"Confirmed"</div> },
                                    )
                                } else {
                                    Either::Right(
                                        view! {
                                            <button
                                                class="btn btn-sm btn-outline"
                                                on:click=move |event| {
                                                    event.prevent_default();
                                                    send_confirmation_action.dispatch(SendEmailConfirmation {});
                                                }
                                            >
                                                {move || {
                                                    if send_confirmation_action.pending().get() {
                                                        Either::Left(
                                                            view! { <span class="loading loading-spinner"></span> },
                                                        )
                                                    } else {
                                                        Either::Right("Send confirmation code")
                                                    }
                                                }}
                                            </button>

                                            <EmailConfirmationModal is_open=show_confirmation_modal />
                                        },
                                    )
                                }}
                            }
                        })
                    } />
                </div>
            </section>

            <section class="my-6">
                <h2 class="h2">"Change email"</h2>

                <ChangeEmailForm on_success=move |_| {
                    send_confirmation_action.dispatch(SendEmailConfirmation {});
                } />
            </section>
        </AuthenticatedPage>
    }
}

#[component]
fn EmailConfirmationModal(is_open: RwSignal<bool>) -> impl IntoView {
    let navigate = use_navigate();
    let mut toast = use_toast();
    let action = ServerAction::<ConfirmEmail>::new();
    let action_value = action.value();
    let error_code = Memo::new(move |_| action_value.read().get_param_error("code"));

    Effect::watch(
        move || action_value.get(),
        move |action_value, _, _| {
            if action_value.is_success() {
                toast.push_alert(AlertType::Success, "Email confirmed successfully");
                navigate("/", Default::default());
            }
        },
        false,
    );

    view! {
        <Modal is_closable=false is_open=is_open>
            <h3 class="h3">"Confirm email"</h3>

            <ActionForm action=action attr:class="form" attr:autocomplete="off" attr:novalidate="true">
                <Show when=move || action_value.read().has_errors()>
                    <Alert alert_type=AlertType::Error>"Failed to confirm email"</Alert>
                </Show>

                <TextField disabled=action.pending() label="Confirmation code" name="code" error=error_code />

                <SubmitButton is_pending=action.pending() />
            </ActionForm>
        </Modal>
    }
}

#[component]
fn ChangeEmailForm(#[prop(into)] on_success: Callback<()>) -> impl IntoView {
    let current_user_resource = use_current_user_resource();
    let mut toast = use_toast();
    let action = ServerAction::<UpdateEmail>::new();
    let action_value = action.value();
    let error_email = Memo::new(move |_| action_value.read().get_param_error("email"));
    let error_password = Memo::new(move |_| action_value.read().get_param_error("password"));

    Effect::watch(
        move || action_value.get(),
        move |action_value, _, _| {
            if action_value.is_success() {
                current_user_resource.refetch();
                on_success.run(());
                toast.push_alert(AlertType::Success, "Email updated successfully");
            }
        },
        false,
    );

    view! {
        <ActionForm action=action attr:class="form" attr:autocomplete="off" attr:novalidate="true">
            <Show when=move || action_value.read().has_errors()>
                <Alert alert_type=AlertType::Error>"Failed to update email"</Alert>
            </Show>

            <TextField disabled=action.pending() label="Email" name="email" error=error_email />

            <PasswordField disabled=action.pending() label="Password" name="password" error=error_password />

            <SubmitButton is_pending=action.pending() />
        </ActionForm>
    }
}
