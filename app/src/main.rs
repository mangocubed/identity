use dioxus::prelude::*;

use sdk::app::components::AppProvider;
use sdk::app::hooks::use_resource_with_spinner;

mod constants;
mod hooks;
mod layouts;
mod local_data;
mod pages;
mod presenters;
mod requests;
mod routes;

#[cfg(feature = "server")]
mod server;

use routes::Routes;

const FAVICON_ICO: Asset = asset!("assets/favicon.ico");
const STYLE_CSS: Asset = asset!("assets/style.css");

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use axum::routing::{delete, get, post};

    use constants::*;
    use server::requests;

    let _ = tokio::join!(
        sdk::app::launch_request_server(|router| {
            router
                .route(PATH_API_AUTHORIZE, post(requests::post_authorize))
                .route(PATH_API_CAN_REGISTER, get(requests::get_can_register))
                .route(PATH_API_CURRENT_USER, get(requests::get_current_user))
                .route(PATH_API_LOGIN, post(requests::post_login))
                .route(PATH_API_LOGOUT, delete(requests::delete_logout))
                .route(PATH_API_REGISTER, post(requests::post_register))
        }),
        tokio::task::spawn_blocking(|| {
            dioxus::launch(App);
        }),
    );
}

#[cfg(not(feature = "server"))]
fn main() {
    use sdk::app::set_request_bearer;

    use crate::local_data::get_session_token;

    if let Some(session_token) = get_session_token() {
        set_request_bearer(&session_token);
    }

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut is_starting = use_signal(|| true);
    let current_user = use_resource_with_spinner("current-user", async || requests::current_user().await.ok());

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
