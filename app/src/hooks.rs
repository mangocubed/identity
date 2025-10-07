use dioxus::prelude::*;

use crate::presenters::UserPresenter;

pub fn use_current_user() -> Resource<Option<UserPresenter>> {
    use_context()
}
