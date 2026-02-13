use leptos::either::EitherOf3;
use leptos::prelude::*;

use crate::icons::{CheckCircleOutline, ExclamationOutline, InformationCircleOutline};

mod current_user;
mod form;
mod logo;
mod modal;
mod navbar;

pub use current_user::CurrentUser;
pub use form::*;
pub use logo::Mango3Logo;
pub use modal::*;
pub use navbar::Navbar;

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
