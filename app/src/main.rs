use dioxus::prelude::*;

use sdk::components::{AppProvider, LoadingOverlay};

mod layouts;
mod pages;
mod routes;

use routes::Routes;

const FAVICON_ICO: Asset = asset!("assets/favicon.ico");
const STYLE_CSS: Asset = asset!("assets/style.css");

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    dioxus::logger::initialize_default();

    let app = axum::Router::new().serve_dioxus_application(ServeConfig::new().unwrap(), App);

    let addr = dioxus::cli_config::fullstack_address_or_localhost();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut app_is_loading = use_signal(|| true);

    use_effect(move || {
        app_is_loading.set(false);
    });

    rsx! {
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1, maximum-scale=1, user-scalable=0",
        }
        document::Link { rel: "icon", href: FAVICON_ICO }
        document::Link { rel: "stylesheet", href: STYLE_CSS }

        AppProvider { Router::<Routes> {} }

        LoadingOverlay { is_visible: app_is_loading }
    }
}
