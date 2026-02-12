#[cfg(feature = "ssr")]
use tower_sessions::SessionManagerLayer;
#[cfg(feature = "ssr")]
use tower_sessions::service::PrivateCookie;
#[cfg(feature = "ssr")]
use tower_sessions_redis_store::{RedisStore, fred};

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

    Ok((
        SessionManagerLayer::new(session_store)
            .with_domain(SESSION_CONFIG.domain.clone())
            .with_expiry(Expiry::OnInactivity(Duration::days(30)))
            .with_http_only(true)
            .with_name("identity_session")
            .with_private(Key::from(SESSION_CONFIG.private_key.as_bytes()))
            .with_same_site(SameSite::Strict)
            .with_secure(SESSION_CONFIG.secure),
        redis_conn,
    ))
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::net::SocketAddr;

    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};

    use identity_app::app::{App, shell};

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
        .layer(session_layer)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    session_redis_conn.await??;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
