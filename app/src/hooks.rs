use dioxus::prelude::*;

use sdk::app::hooks::use_resource_with_spinner;

use crate::presenters::UserPresenter;
use crate::server_fns;

pub fn use_can_register() -> Resource<bool> {
    use_resource_with_spinner("can-register", || async {
        server_fns::can_register().await.unwrap_or_default()
    })
}

pub fn use_current_user() -> Resource<Option<UserPresenter>> {
    use_context()
}
