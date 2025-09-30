use dioxus::prelude::*;

use crate::layouts::UserLayout;
use crate::pages::*;

#[derive(Clone, Routable)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
pub enum Routes {
    #[layout(UserLayout)]
        #[route("/")]
        HomePage {},
}

impl Routes {
    pub fn home() -> Self {
        Self::HomePage {}
    }
}
