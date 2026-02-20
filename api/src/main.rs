use std::net::SocketAddr;

use axum::Router;
use axum::http::Method;
use axum::routing::{get, post};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::Level;

use identity_core::config::API_CONFIG;

mod constants;
mod handlers;
mod params;

use handlers::{get_current_user, get_index, get_user, get_user_avatar_image, post_oauth_revoke, post_oauth_token};

#[tokio::main]
async fn main() {
    let tracing_level = if cfg!(debug_assertions) {
        Level::DEBUG
    } else {
        Level::INFO
    };

    tracing_subscriber::fmt().with_max_level(tracing_level).init();

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let router = Router::new()
        .route("/", get(get_index))
        .route("/current-user", get(get_current_user))
        .route("/oauth/revoke", post(post_oauth_revoke))
        .route("/oauth/token", post(post_oauth_token))
        .route("/users/{username_or_id}", get(get_user))
        .route("/users/{username_or_id}/avatar-image", get(get_user_avatar_image))
        .layer(cors_layer)
        .layer(TraceLayer::new_for_http());

    let api_address = &API_CONFIG.address;

    let listener = TcpListener::bind(&api_address).await.unwrap();

    tracing::info!("Listening on http://{api_address}");

    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
