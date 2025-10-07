use dioxus::prelude::*;

use sdk::components::AppProvider;
use sdk::hooks::use_resource_with_loader;

#[cfg(not(feature = "server"))]
use sdk::serv_fn::{remove_serv_fn_header, set_serv_fn_header};
#[cfg(not(feature = "server"))]
use sdk::{DataStorage, data_storage};

mod constants;
mod hooks;
mod layouts;
mod pages;
mod presenters;
mod routes;
mod server_fns;

use routes::Routes;
use server_fns::get_current_user;

#[cfg(not(feature = "server"))]
use constants::{HEADER_AUTHORIZATION, KEY_SESSION_TOKEN};

const FAVICON_ICO: Asset = asset!("assets/favicon.ico");
const STYLE_CSS: Asset = asset!("assets/style.css");

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use std::net::SocketAddr;

    dioxus::logger::initialize_default();

    let app = axum::Router::new().serve_dioxus_application(ServeConfig::new().unwrap(), App);

    let addr = dioxus::cli_config::fullstack_address_or_localhost();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

#[cfg(not(feature = "server"))]
fn main() {
    if let Some(session_token) = data_storage().get(KEY_SESSION_TOKEN) {
        set_serv_fn_header(HEADER_AUTHORIZATION, &format!("Bearer {session_token}"));
    }

    dioxus::launch(App);
}

#[cfg(feature = "server")]
fn delete_session_token() {}

#[cfg(not(feature = "server"))]
fn delete_session_token() {
    data_storage().delete(KEY_SESSION_TOKEN);
    remove_serv_fn_header(HEADER_AUTHORIZATION);
}

#[cfg(feature = "server")]
fn set_session_token(_token: &str) {}

#[cfg(not(feature = "server"))]
fn set_session_token(token: &str) {
    data_storage().set(KEY_SESSION_TOKEN, token);
    set_serv_fn_header(HEADER_AUTHORIZATION, &format!("Bearer {token}"));
}

#[component]
fn App() -> Element {
    let mut is_starting = use_signal(|| true);
    let current_user = use_resource_with_loader("current-user".to_owned(), async || {
        get_current_user().await.ok().flatten()
    });

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
