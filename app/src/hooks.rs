use dioxus::prelude::*;

use sdk::app::ServerResult;
use sdk::app::hooks::use_resource_with_spinner;

use crate::presenters::UserPresenter;
use crate::requests;

pub fn use_can_register() -> Resource<ServerResult<bool>> {
    use_resource_with_spinner("can-register", requests::can_register)
}

pub fn use_current_user() -> Resource<Option<UserPresenter>> {
    use_context()
}
