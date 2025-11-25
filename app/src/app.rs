use dioxus::CapturedError;
use dioxus::prelude::*;

use sdk::app::components::AppProvider;
use sdk::app::hooks::use_resource_with_spinner;
use sdk::app::run_with_spinner;

use crate::constants::{FAVICON_ICO, STYLE_CSS};
use crate::routes::Routes;
use crate::server_fns;
use crate::storage::delete_session;
use crate::storage::get_session;
use crate::storage::set_session;

#[component]
pub fn App() -> Element {
    let mut is_starting = use_signal(|| true);
    let mut current_user = use_resource_with_spinner("current-user", move || async move {
        if get_session().is_none() {
            return Err(CapturedError::msg("Unauthenticated".to_owned()));
        }

        server_fns::current_user().await
    });

    use_context_provider(|| current_user);

    use_future(move || async move {
        if !get_session()
            .map(|session| session.should_refresh())
            .unwrap_or_default()
        {
            return;
        }

        let result = run_with_spinner("refresh-session", server_fns::refresh_session).await;

        if let Ok(session) = result {
            set_session(&session);
        } else {
            delete_session();
            current_user.restart();
        }
    });

    use_effect(move || {
        if current_user.read().is_some() {
            is_starting.set(false);
        }
    });

    rsx! {
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1, maximum-scale=1, user-scalable=0",
        }
        document::Link { rel: "icon", href: FAVICON_ICO }
        document::Link { rel: "stylesheet", href: STYLE_CSS }

        AppProvider { is_starting, Router::<Routes> {} }
    }
}
