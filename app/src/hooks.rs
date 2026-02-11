use leptos::prelude::*;

use crate::components::AlertType;

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

pub fn provide_toast() -> Toast {
    let toast = Toast::default();

    provide_context(toast);

    toast
}

pub fn use_toast() -> Toast {
    expect_context()
}
