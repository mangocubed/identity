mod app;
mod components;
mod constants;
mod hooks;
mod layouts;
mod pages;
mod presenters;
mod requests;
mod routes;
mod server_fns;
mod storage;

#[cfg(feature = "server")]
mod server;

use app::App;

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use std::net::SocketAddr;

    use axum::routing::post;
    use dioxus::prelude::{DioxusRouterExt, ServeConfig};

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

    use crate::storage::get_session;

    if let Some(session) = get_session() {
        set_request_bearer(&session.token);
    }

    sdk::app::launch(App);
}
