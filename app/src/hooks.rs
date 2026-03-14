use leptos::prelude::*;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;

use crate::components::AlertType;
use crate::presenters::UserPresenter;
use crate::server_fns::{self, ServerFnResult};

#[derive(Clone, Default, Params, PartialEq)]
pub struct LoginQuery {
    redirect_to: Option<String>,
}

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

pub fn provide_current_user_resource() -> Resource<ServerFnResult<UserPresenter>> {
    let resource = Resource::new_blocking(|| (), |_| server_fns::current_user());

    provide_context(resource);

    resource
}

pub fn provide_toast() -> Toast {
    let toast = Toast::default();

    provide_context(toast);

    toast
}

pub fn use_current_user_resource() -> Resource<ServerFnResult<UserPresenter>> {
    expect_context()
}

pub fn use_redirect_to() -> Memo<String> {
    let query = use_query::<LoginQuery>();

    Memo::new(move |_| {
        query.with(|result| {
            let value = result
                .as_ref()
                .ok()
                .and_then(|query| query.redirect_to.clone())
                .unwrap_or("/".to_owned());

            if value.starts_with("/login") || value.starts_with("/register") {
                "/".to_owned()
            } else {
                value
            }
        })
    })
}

pub fn use_toast() -> Toast {
    expect_context()
}
