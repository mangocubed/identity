use dioxus::prelude::*;

use crate::layouts::{GuestLayout, UserLayout};
use crate::pages::*;

#[derive(Clone, Routable)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
pub enum Routes {
    #[layout(UserLayout)]
        #[route("/")]
        HomePage {},
    #[end_layout]

    #[layout(GuestLayout)]
        #[route("/register")]
        RegisterPage {},
}

impl Routes {
    pub fn home() -> Self {
        Self::HomePage {}
    }
}
