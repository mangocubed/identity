use dioxus::prelude::*;

use sdk::components::AppProvider;
use sdk::hooks::use_resource_with_loader;

#[cfg(not(feature = "server"))]
use sdk::serv_fn::set_serv_fn_header;

mod constants;
mod hooks;
mod layouts;
mod local_data;
mod pages;
mod presenters;
mod routes;
mod server_fns;

use routes::Routes;
use server_fns::get_current_user;

const FAVICON_ICO: Asset = asset!("assets/favicon.ico");
const STYLE_CSS: Asset = asset!("assets/style.css");

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use std::net::SocketAddr;

    use axum::routing::{get, post};

    use routes::priv_api;

    dioxus::logger::initialize_default();

    let app = axum::Router::new()
        .route("/priv-api/refresh-auth", post(priv_api::post_refresh_auth))
        .route("/priv-api/user-info", get(priv_api::get_user_info))
        .route("/priv-api/verify-auth", get(priv_api::get_verify_auth))
        .serve_dioxus_application(ServeConfig::new().unwrap(), App);

    let addr = dioxus::cli_config::fullstack_address_or_localhost();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

#[cfg(not(feature = "server"))]
fn main() {
    use crate::constants::HEADER_AUTHORIZATION;
    use crate::local_data::get_session_token;

    if let Some(session_token) = get_session_token() {
        set_serv_fn_header(HEADER_AUTHORIZATION, &format!("Bearer {session_token}"));
    }

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut is_starting = use_signal(|| true);
    let current_user = use_resource_with_loader("current-user", async || get_current_user().await.ok().flatten());

    use_context_provider(|| current_user);

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
