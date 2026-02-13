#[cfg(feature = "ssr")]
use axum::extract::{Path, Query};
#[cfg(feature = "ssr")]
use axum::response::IntoResponse;
#[cfg(feature = "ssr")]
use serde::Deserialize;
#[cfg(feature = "ssr")]
use tower_sessions::SessionManagerLayer;
#[cfg(feature = "ssr")]
use tower_sessions::service::PrivateCookie;
#[cfg(feature = "ssr")]
use tower_sessions_redis_store::{RedisStore, fred};
#[cfg(feature = "ssr")]
use uuid::Uuid;

#[cfg(feature = "ssr")]
async fn session_layer() -> anyhow::Result<(
    SessionManagerLayer<RedisStore<fred::prelude::Pool>, PrivateCookie>,
    fred::types::ConnectHandle,
)> {
    use fred::prelude::{ClientLike, Config, Pool};
    use time::Duration;
    use tower_sessions::Expiry;
    use tower_sessions::cookie::{Key, SameSite};

    use identity_app::config::SESSION_CONFIG;

    let redis_pool = Pool::new(Config::from_url(&SESSION_CONFIG.redis_url)?, None, None, None, 6)?;

    let redis_conn = redis_pool.connect();

    redis_pool.wait_for_connect().await?;

    let session_store = RedisStore::new(redis_pool);

    let mut session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(Duration::days(30)))
        .with_http_only(true)
        .with_name("identity_session")
        .with_private(Key::from(SESSION_CONFIG.private_key.as_bytes()))
        .with_same_site(SameSite::Strict)
        .with_secure(SESSION_CONFIG.secure);

    if let Some(domain) = &SESSION_CONFIG.domain {
        session_layer = session_layer.with_domain(domain);
    }

    Ok((session_layer, redis_conn))
}

#[cfg(feature = "ssr")]
#[derive(Deserialize)]
pub struct AvatarImageParams {
    pub size: Option<u32>,
}
#[cfg(feature = "ssr")]
async fn get_user_avatar_image(
    Path(user_id): Path<Uuid>,
    Query(params): Query<AvatarImageParams>,
) -> impl IntoResponse {
    use axum::body::Body;
    use http::StatusCode;
    use http::header::{CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_TYPE};

    use identity_core::commands;

    let size = params.size.unwrap_or(128);

    if size > 512 {
        return Err((StatusCode::BAD_REQUEST, "BAD REQUEST"));
    }

    let Ok(user) = commands::get_user_by_id(user_id).await else {
        return Err((StatusCode::NOT_FOUND, "NOT FOUND"));
    };

    let Ok(avatar_image) = user.avatar_image(size) else {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL SERVER ERROR"));
    };

    let content_length = avatar_image.len();
    let body = Body::from(avatar_image);

    let headers = [
        (CONTENT_TYPE, "image/jpeg".to_owned()),
        (CONTENT_LENGTH, content_length.to_string()),
        (
            CONTENT_DISPOSITION,
            format!("inline; filename=\"{}_{}x{}.jpg\"", user_id, size, size),
        ),
    ];

    Ok((headers, body))
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::net::SocketAddr;

    use axum::Router;
    use axum::routing::get;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use tower_http::trace::TraceLayer;
    use tracing::Level;

    use identity_app::app::{App, shell};
    use identity_app::config::APP_CONFIG;

    let tracing_level = if cfg!(debug_assertions) {
        Level::DEBUG
    } else {
        Level::INFO
    };

    tracing_subscriber::fmt().with_max_level(tracing_level).init();

    let conf = get_configuration(None)?;
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    let (session_layer, session_redis_conn) = session_layer().await?;

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .route("/storage/users/{user_id}/avatar_image", get(get_user_avatar_image))
        .layer(session_layer)
        .layer(TraceLayer::new_for_http())
        .layer(APP_CONFIG.client_ip_source.clone().into_extension())
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    session_redis_conn.await??;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
