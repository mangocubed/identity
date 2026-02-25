use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

use crate::components::AlertType;
use crate::hooks::{use_current_user_resource, use_toast};
use crate::icons::{ChevronDownMini, Mango3Icon};
use crate::server_fns::{ActionResultExt, FinishSession};

use super::{ConfirmationModal, CurrentUser, Mango3Logo};

#[component]
pub fn Navbar() -> impl IntoView {
    let navigate = use_navigate();
    let mut toast = use_toast();
    let current_user_resource = use_current_user_resource();
    let show_logout_confirmation = RwSignal::new(false);
    let logout_action = ServerAction::<FinishSession>::new();
    let logout_action_value = logout_action.value();

    Effect::watch(
        move || logout_action_value.get(),
        move |action_value, _, _| {
            if action_value.is_success() {
                current_user_resource.refetch();
                toast.push_alert(AlertType::Success, "Session finished successfully");
                navigate("/login", Default::default());
            }
        },
        false,
    );

    let brand_dev = {
        if cfg!(debug_assertions) {
            Some(view! { <div class="brand-dev">"(dev)"</div> })
        } else {
            None
        }
    };

    view! {
        <div class="navbar">
            <div class="navbar-start">
                <A href="/">
                    <div class="brand">
                        <Mango3Icon class="brand-icon" />

                        <Mango3Logo class="brand-logo" />

                        <div class="brand-suffix">"ID"</div>

                        {brand_dev}
                    </div>
                </A>
            </div>

            <div class="navbar-end">
                <CurrentUser children=move |user| {
                    if let Ok(user) = user {
                        Either::Left(
                            view! {
                                <div class="dropdown dropdown-end">
                                    <button class="btn btn-ghost text-sm normal-case px-1" tabindex="0">
                                        <div class="avatar">
                                            <div class="rounded-full">
                                                <img
                                                    alt=user.initials.clone()
                                                    src=user.avatar_image_url(32).to_string()
                                                />
                                            </div>
                                        </div>

                                        <div class="text-left ml-2">
                                            <div class="font-bold">{user.display_name}</div>
                                            <div class="opacity-70">"@"{user.username}</div>
                                        </div>

                                        <ChevronDownMini />
                                    </button>

                                    <ul tabindex="0" class="dropdown-content mt-4 menu">
                                        <li>
                                            <a on:click=move |event| {
                                                event.prevent_default();
                                                show_logout_confirmation.set(true);
                                            }>"Logout"</a>
                                        </li>
                                    </ul>
                                </div>

                                <ConfirmationModal
                                    is_open=show_logout_confirmation
                                    on_accept=move |_| {
                                        logout_action.dispatch(FinishSession {});
                                    }
                                >
                                    "Are you sure you want to logout?"
                                </ConfirmationModal>
                            },
                        )
                    } else {
                        Either::Right(
                            view! {
                                <A attr:class="btn btn-outline normal-case" href="/login">
                                    "Login"
                                </A>
                            },
                        )
                    }
                } />
            </div>
        </div>
    }
}
