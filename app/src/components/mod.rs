use leptos::either::EitherOf3;
use leptos::prelude::*;
use leptos_router::components::Redirect;
use leptos_router::hooks::use_url;

use crate::hooks::use_current_user_resource;
use crate::icons::{CheckCircleOutline, ExclamationOutline, InformationCircleOutline};

mod form;
mod logo;

pub use form::*;
pub use logo::Mango3Logo;

#[derive(Clone, Copy, Default)]
pub enum AlertType {
    Error,
    #[default]
    None,
    Success,
}

impl AlertType {
    fn class_name(&self) -> &'static str {
        match self {
            AlertType::Error => "alert-error",
            AlertType::None => "",
            AlertType::Success => "alert-success",
        }
    }
}

#[component]
pub fn Alert(#[prop(optional)] alert_type: AlertType, children: Children) -> impl IntoView {
    view! {
        <div class=format!("alert my-2 {}", alert_type.class_name()) role="alert">
            {move || match alert_type {
                AlertType::Error => EitherOf3::A(view! { <ExclamationOutline /> }),
                AlertType::None => EitherOf3::B(view! { <InformationCircleOutline /> }),
                AlertType::Success => EitherOf3::C(view! { <CheckCircleOutline /> }),
            }}

            <div>{children()}</div>
        </div>
    }
}

#[component]
pub fn RequireAuthentication() -> impl IntoView {
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
    }
}

#[component]
pub fn RequireNoAuthentication() -> impl IntoView {
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
    }
}
