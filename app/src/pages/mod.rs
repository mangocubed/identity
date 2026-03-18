use leptos::either::Either;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{components::Redirect, hooks::use_url};

mod authorize_page;
mod change_password_page;
mod home_page;
mod home_parent_page;
mod login_page;
mod register_page;

pub use authorize_page::AuthorizePage;
pub use change_password_page::ChangePasswordPage;
pub use home_page::HomePage;
pub use home_parent_page::HomeParentPage;
pub use login_page::LoginPage;
pub use register_page::RegisterPage;
use url::form_urlencoded;

use crate::hooks::{use_current_user_resource, use_redirect_to};

#[component]
pub fn AuthenticatedPage(children: ChildrenFn, #[prop(into)] title: String) -> impl IntoView {
    let current_user_resource = use_current_user_resource();
    let url = use_url();
    let redirect_to = Memo::new(move |_| {
        url.with(|url| {
            let mut full_path = url.path().to_owned();

            if !url.search().is_empty() {
                full_path.push('?');
                full_path.push_str(url.search());
            }

            form_urlencoded::byte_serialize(full_path.as_bytes()).collect::<String>()
        })
    });
    let children_store = StoredValue::new(children);

    Effect::new(move || {
        current_user_resource.refetch();
    });

    view! {
        <Transition>
            {
                let title = title.clone();
                move || {
                    let title = title.clone();
                    Suspend::new(async move {
                        if let Some(Err(_)) = *current_user_resource.read() {
                            Either::Left(
                                view! {
                                    <Redirect path=format!("/login?redirect_to={}", redirect_to.get_untracked()) />
                                },
                            )
                        } else {
                            Either::Right(
                                view! {
                                    <Title text=title.clone() />

                                    <h1 class="h1">{title}</h1>

                                    {children_store.read_value()()}
                                },
                            )
                        }
                    })
                }
            }
        </Transition>
    }
}

#[component]
pub fn GuestPage(children: ChildrenFn, #[prop(into)] title: String) -> impl IntoView {
    let current_user_resource = use_current_user_resource();
    let redirect_to = use_redirect_to();
    let children_store = StoredValue::new(children);

    Effect::new(move || {
        current_user_resource.refetch();
    });

    view! {
        <Transition>
            {
                let title = title.clone();
                move || {
                    let title = title.clone();
                    Suspend::new(async move {
                        if let Some(Ok(_)) = *current_user_resource.read() {
                            Either::Left(view! { <Redirect path=redirect_to.get_untracked() /> })
                        } else {
                            Either::Right(
                                view! {
                                    <Title text=title.clone() />

                                    <h1 class="h1">{title}</h1>

                                    {children_store.read_value()()}
                                },
                            )
                        }
                    })
                }
            }
        </Transition>
    }
}
