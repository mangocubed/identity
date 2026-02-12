use leptos::prelude::*;

mod home_page;
mod login_page;
mod register_page;

pub use home_page::HomePage;
use leptos_meta::Title;
pub use login_page::LoginPage;
pub use register_page::RegisterPage;

use crate::components::{RequireAuthentication, RequireNoAuthentication};
use crate::hooks::use_current_user_resource;

#[component]
pub fn AuthenticatedPage(children: Children, #[prop(into)] title: String) -> impl IntoView {
    view! {
        <RequireAuthentication />

        <Page title=title>{children()}</Page>
    }
}

#[component]
pub fn GuestPage(children: Children, #[prop(into)] title: String) -> impl IntoView {
    view! {
        <RequireNoAuthentication />

        <Page title=title.clone()>
            <h1 class="h1">{title}</h1>

            {children()}
        </Page>
    }
}

#[component]
pub fn Page(children: Children, #[prop(into)] title: String) -> impl IntoView {
    let current_user_resource = use_current_user_resource();

    Effect::new(move || {
        current_user_resource.refetch();
    });

    view! {
        <Title text=title />

        {children()}
    }
}
