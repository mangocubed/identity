use leptos::prelude::*;
use leptos::server::codee::string::FromToStringCodec;
use leptos_use::{SameSite, UseCookieOptions, use_cookie_with_options};

use crate::components::AlertType;
use crate::constants::KEY_REDIRECT_TO;
use crate::presenters::UserPresenter;
use crate::server_fns::{self, ServerFnResult};

#[derive(Clone, Copy, Default)]
pub struct Toast {
    alerts: RwSignal<Vec<(usize, AlertType, String)>>,
    id_seq: usize,
}

impl Toast {
    pub fn alerts(&self) -> Vec<(usize, AlertType, String)> {
        self.alerts.get()
    }

    pub fn push_alert(&mut self, alert_type: AlertType, message: &str) {
        self.alerts.update(|alerts| {
            alerts.push((self.id_seq, alert_type, message.to_owned()));
            self.id_seq += 1;
        })
    }

    pub fn remove_alert(&mut self, id: usize) {
        self.alerts.update(|alerts| {
            alerts.retain(|(alert_id, _, _)| *alert_id != id);
        })
    }
}

pub fn provide_current_user_resource() {
    provide_context(Resource::new_blocking(|| (), |_| server_fns::current_user()))
}

pub fn provide_toast() -> Toast {
    let toast = Toast::default();

    provide_context(toast);

    toast
}

pub fn use_current_user_resource() -> Resource<ServerFnResult<UserPresenter>> {
    expect_context()
}

pub fn use_redirect_to_cookie() -> (Signal<Option<String>>, WriteSignal<Option<String>>) {
    use_cookie_with_options::<String, FromToStringCodec>(
        KEY_REDIRECT_TO,
        UseCookieOptions::default()
            .http_only(true)
            .max_age(3600000)
            .same_site(SameSite::Strict),
    )
}

pub fn use_toast() -> Toast {
    expect_context()
}
