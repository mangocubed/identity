pub mod handlers {
    use axum::Json;
    use axum::http::StatusCode;
    use axum::http::header::HeaderMap;
    use axum::response::{IntoResponse, Result};
    use chrono::{Duration, Utc};

    use sdk::app::{extract_bearer, require_app_token};
    use sdk::constants::*;

    use identity_core::commands;
    use identity_core::models::{Session, User};

    use crate::requests::AuthorizeParams;

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
}
