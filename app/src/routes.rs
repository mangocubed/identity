use dioxus::prelude::*;
use uuid::Uuid;

use crate::layouts::{LoginLayout, UserLayout};
use crate::pages::*;

#[derive(Clone, Routable)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
pub enum Routes {
    #[layout(UserLayout)]
        #[route("/")]
        HomePage {},
        #[route("/authorize?:client_id")]
        AuthorizePage { client_id: Uuid },
        #[route("/change-password")]
        ChangePasswordPage {},
    #[end_layout]

    #[layout(LoginLayout)]
        #[route("/login")]
        LoginPage {},
        #[route("/register")]
        RegisterPage {},
}

impl Routes {
    pub fn change_password() -> Self {
        Self::ChangePasswordPage {}
    }

    pub fn home() -> Self {
        Self::HomePage {}
    }

    pub fn login() -> Self {
        Self::LoginPage {}
    }

    pub fn register() -> Self {
        Self::RegisterPage {}
    }
}
