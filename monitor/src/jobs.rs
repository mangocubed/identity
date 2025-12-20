use std::borrow::Cow;

use base64::Engine;
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::Sha256;

use identity_core::commands;
use identity_core::config::IP_GEOLOCATION_CONFIG;
use identity_core::jobs_storage::*;

use crate::ApalisError;
use crate::mailer::*;

#[derive(Deserialize)]
struct Location<'a> {
    country_code2: Cow<'a, str>,
    state_prov: Cow<'a, str>,
    city: Cow<'a, str>,
}

#[derive(Deserialize)]
struct IpGeo<'a> {
    location: Location<'a>,
}

pub async fn finished_session_job(job: FinishedSession) -> Result<(), apalis::prelude::Error> {
    let session = commands::get_finished_session_by_id(job.session_id)
        .await
        .or_apalis_error()?;

    let authorizations = session.authorizations().await;

    for authorization in authorizations {
        let application = authorization.application().await;

        let _ = commands::revoke_authorization(&authorization).await;

        if application.webhook_url.is_some() {
            jobs_storage()
                .await
                .push_webhook_event(
                    &application,
                    "authorization_revoked",
                    serde_json::json!({ "token": authorization.token }),
                )
                .await;
        }
    }

    Ok(())
}

pub async fn new_confirmation_job(job: NewConfirmation) -> Result<(), apalis::prelude::Error> {
    let confirmation = commands::get_confirmation_by_id(job.confirmation_id)
        .await
        .or_apalis_error()?;

    send_new_confirmation_email(&confirmation, &job.code).await
}

pub async fn new_session_job(job: NewSession) -> Result<(), apalis::prelude::Error> {
    let mut session = commands::get_session_by_id(job.session_id).await.or_apalis_error()?;

    if !job.ip_addr.is_loopback() && !job.ip_addr.is_multicast() && !job.ip_addr.is_unspecified() {
        let result = reqwest::get(format!(
            "https://api.ipgeolocation.io/v2/ipgeo?apiKey={}&ip={}",
            IP_GEOLOCATION_CONFIG.api_key, job.ip_addr
        ))
        .await;

        if let Ok(response) = result
            && let Ok(ip_geo) = response.json::<IpGeo>().await
        {
            let result = commands::update_session_location(
                &session,
                &ip_geo.location.country_code2,
                &ip_geo.location.state_prov,
                &ip_geo.location.city,
            )
            .await;

            if let Ok(updated_session) = result {
                session = updated_session
            }
        }
    };

    send_new_session_email(&session).await
}

pub async fn new_user_job(job: NewUser) -> Result<(), apalis::prelude::Error> {
    let user = commands::get_user_by_id(job.user_id).await.or_apalis_error()?;

    let _ = admin_emails::send_new_user_email(&user).await;

    send_welcome_email(&user).await
}

pub async fn password_changed_job(job: PasswordChanged) -> Result<(), apalis::prelude::Error> {
    let user = commands::get_user_by_id(job.user_id).await.or_apalis_error()?;

    send_password_changed_email(&user).await
}

pub async fn refreshed_authorization_job(job: RefreshedAuthorization) -> Result<(), apalis::prelude::Error> {
    let authorization = commands::get_authorization_by_id(job.authorization_id)
        .await
        .or_apalis_error()?;
    let session = authorization.session().await;

    let _ = commands::refresh_session_expiration(&session).await;

    Ok(())
}

pub async fn webhook_event_job(job: WebhookEvent) -> Result<(), apalis::prelude::Error> {
    let application = commands::get_application_by_id(job.application_id)
        .await
        .or_apalis_error()?;

    let Some(webhook_url) = application.webhook_url else {
        return Ok(());
    };

    let mut hmac = Hmac::<Sha256>::new_from_slice(application.webhook_secret.as_bytes()).or_apalis_error()?;

    let message = serde_json::json!({ "event_type": job.event_type, "data": job.data });
    let message_bytes = serde_json::to_string(&message).or_apalis_error()?;

    hmac.update(message_bytes.as_bytes());

    let signature = hmac.finalize();
    let signature_base64 = base64::engine::general_purpose::STANDARD.encode(signature.into_bytes());

    let _ = reqwest::Client::new()
        .post(webhook_url)
        .header("X-Webhook-Signature", signature_base64)
        .json(&message)
        .send()
        .await
        .or_apalis_error()?
        .error_for_status()
        .or_apalis_error()?;

    Ok(())
}
