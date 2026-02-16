use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::{components::Redirect, hooks::use_url};

mod authorize_page;
mod home_page;
mod login_page;
mod register_page;

pub use authorize_page::AuthorizePage;
pub use home_page::HomePage;
pub use login_page::LoginPage;
pub use register_page::RegisterPage;

use crate::hooks::use_current_user_resource;

#[component]
pub fn AuthenticatedPage(children: Children, #[prop(into)] title: String) -> impl IntoView {
    let current_user_resource = use_current_user_resource();
    let url = use_url();
    let redirect_to = Memo::new(move |_| {
        url.with(|url| {
            let mut full_path = url.path().to_owned();

            if !url.search().is_empty() {
                full_path.push('?');
                full_path.push_str(url.search());
            }

            full_path
        })
    });

    #[cfg(not(feature = "ssr"))]
    let (_, set_redirect_to_cookie) = crate::hooks::use_redirect_to_cookie();

    let update_redirect_to = move || {
        #[cfg(not(feature = "ssr"))]
        set_redirect_to_cookie.set(Some(redirect_to.get()));

        #[cfg(feature = "ssr")]
        {
            use cookie::Cookie;
            use http::HeaderValue;
            use http::header::SET_COOKIE;
            use leptos_axum::ResponseOptions;

            use crate::constants::KEY_REDIRECT_TO;

            if let Some(response_options) = use_context::<ResponseOptions>() {
                let cookie: Cookie = Cookie::build((KEY_REDIRECT_TO, redirect_to.get()))
                    .http_only(true)
                    .max_age(cookie::time::Duration::hours(1))
                    .into();
                if let Ok(header_value) = HeaderValue::from_str(&cookie.encoded().to_string()) {
                    response_options.append_header(SET_COOKIE, header_value);
                }
            }
        }
    };

    view! {
        <Suspense>
            {move || Suspend::new(async move {
                if let Some(Err(_)) = *current_user_resource.read() {
                    update_redirect_to();
                    Some(view! { <Redirect path="/login" /> })
                } else {
                    None
                }
            })}
        </Suspense>

        <Page title=title>{children()}</Page>
    }
}

#[component]
pub fn GuestPage(children: Children, #[prop(into)] title: String) -> impl IntoView {
    let current_user_resource = use_current_user_resource();

    view! {
        <Suspense>
            {move || Suspend::new(async move {
                if let Some(Ok(_)) = *current_user_resource.read() {
                    Some(view! { <Redirect path="/" /> })
                } else {
                    None
                }
            })}
        </Suspense>

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
