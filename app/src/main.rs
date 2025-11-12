use dioxus::CapturedError;
use dioxus::prelude::*;

use sdk::app::components::AppProvider;
use sdk::app::hooks::use_resource_with_spinner;
use sdk::app::{get_request_bearer, run_with_spinner};

mod components;
mod constants;
mod hooks;
mod layouts;
mod local_data;
mod pages;
mod presenters;
mod requests;
mod routes;
mod server_fns;

#[cfg(feature = "server")]
mod server;

use local_data::{get_session, set_session};
use routes::Routes;

const FAVICON_ICO: Asset = asset!("assets/favicon.ico");
const STYLE_CSS: Asset = asset!("assets/style.css");

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use std::net::SocketAddr;

    use axum::routing::post;

    use constants::*;
    use server::handlers::*;

    let _ = tokio::join!(
        sdk::app::launch_request_server(|router| { router.route(PATH_API_AUTHORIZE, post(post_authorize)) }),
        async {
            let address = dioxus::cli_config::fullstack_address_or_localhost();

            let router = axum::Router::new().serve_dioxus_application(ServeConfig::new(), App);

            let listener = tokio::net::TcpListener::bind(address).await.unwrap();

            axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())
                .await
                .unwrap();
        },
    );
}

#[cfg(not(feature = "server"))]
fn main() {
    use sdk::app::set_request_bearer;

    if let Some(session) = get_session() {
        set_request_bearer(&session.token);
    }

    sdk::app::launch(App);
}

#[component]
fn App() -> Element {
    let mut is_starting = use_signal(|| true);
    let current_user = use_resource_with_spinner("current-user", move || async move {
        if get_request_bearer().is_none() {
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
            set_session(session);
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
