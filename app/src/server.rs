pub mod handlers {
    use std::net::{IpAddr, SocketAddr};

    use axum::Json;
    use axum::extract::ConnectInfo;
    use axum::http::StatusCode;
    use axum::http::header::HeaderMap;
    use axum::response::{IntoResponse, Result};
    use chrono::{Duration, Utc};

    use sdk::app::{ActionError, ActionSuccess, extract_bearer, require_app_token};
    use sdk::constants::*;

    use identity_core::commands;
    use identity_core::inputs::LoginInput;
    use identity_core::models::{Session, User};

    use crate::requests::AuthorizeParams;

    fn extract_client_ip_addr(headers: &HeaderMap, connect_info: ConnectInfo<SocketAddr>) -> IpAddr {
        let real_ip = headers
            .get(HEADER_X_REAL_IP)
            .and_then(|ip| ip.to_str().ok())
            .and_then(|ip| ip.parse().ok());

        if let Some(real_ip) = real_ip {
            return real_ip;
        }

        connect_info.0.ip()
    }

    fn extract_user_agent(headers: &HeaderMap) -> String {
        headers
            .get(HEADER_USER_AGENT)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("Unknown")
            .to_owned()
    }

    async fn extract_session<'a>(headers: &HeaderMap) -> Result<Session<'a>, (StatusCode, &'a str)> {
        let bearer = extract_bearer(headers)?;

        commands::get_session_by_token(bearer.token())
            .await
            .map_err(|_| RESPONSE_UNAUTHORIZED)
    }

    async fn extract_user<'a>(headers: &HeaderMap) -> Result<User<'a>, (StatusCode, &'a str)> {
        let bearer = extract_bearer(headers)?;

        commands::get_user_by_session_token(bearer.token())
            .await
            .map_err(|_| RESPONSE_UNAUTHORIZED)
    }

    async fn require_no_session<'a>(headers: &HeaderMap) -> Result<(), (StatusCode, &'a str)> {
        require_app_token(headers).await?;

        if extract_session(headers).await.is_err() {
            Ok(())
        } else {
            Err(RESPONSE_FORBIDDEN)
        }
    }

    pub async fn delete_logout(headers: HeaderMap) -> Result<impl IntoResponse> {
        let session = extract_session(&headers).await?;

        commands::finish_session(&session)
            .await
            .map_err(|_| RESPONSE_INTERNAL_SERVER_ERROR)?;

        Ok(StatusCode::NO_CONTENT)
    }

    pub async fn post_authorize(headers: HeaderMap, Json(params): Json<AuthorizeParams>) -> Result<impl IntoResponse> {
        require_app_token(&headers).await?;

        let (application, session, user) = tokio::try_join!(
            async {
                commands::get_application_by_id(params.client_id)
                    .await
                    .map_err(|_| RESPONSE_NOT_FOUND)
            },
            extract_session(&headers),
            extract_user(&headers),
        )?;

        let authorization =
            commands::insert_or_refresh_authorization(&application, &user, &session, Utc::now() + Duration::hours(1))
                .await
                .map_err(|_| RESPONSE_INTERNAL_SERVER_ERROR)?;

        let mut redirect_url = application.redirect_url();

        redirect_url.set_query(Some(&format!(
            "token={}&expires_at={}",
            authorization.token, authorization.expires_at
        )));

        Ok((StatusCode::CREATED, format!("\"{redirect_url}\"")))
    }

    pub async fn post_login(
        headers: HeaderMap,
        connect_info: ConnectInfo<SocketAddr>,
        Json(input): Json<LoginInput>,
    ) -> impl IntoResponse {
        require_no_session(&headers).await?;

        let user = commands::authenticate_user(&input)
            .await
            .map_err(|errors| ActionError::new("Failed to authenticate user", Some(errors)))?;

        let user_agent = extract_user_agent(&headers);
        let ip_addr = extract_client_ip_addr(&headers, connect_info);

        let result = commands::insert_session(&user, &user_agent, ip_addr).await;

        match result {
            Ok(session) => Ok(ActionSuccess::new(
                "User authenticated successfully",
                session.token.into(),
            )),
            Err(_) => Err(ActionError::new("Failed to authenticate user", None)),
        }
    }
}
