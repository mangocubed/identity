use std::borrow::Cow;

use serde::Deserialize;

use identity_core::commands;
use identity_core::config::IP_GEOLOCATION_CONFIG;
use identity_core::jobs_storage::{FinishedSession, NewSession, NewUser};

use crate::mailer::{send_new_session_email, send_new_user_email};

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
    let session = commands::get_session_by_id(job.session_id)
        .await
        .expect("Could not get session");

    let _ = commands::revoke_authorizations_by_session(&session).await;

    Ok(())
}

pub async fn new_user_job(job: NewUser) -> Result<(), apalis::prelude::Error> {
    let user = commands::get_user_by_id(job.user_id).await.expect("Could not get user");

    send_new_user_email(&user).await
}

pub async fn new_session_job(job: NewSession) -> Result<(), apalis::prelude::Error> {
    let mut session = commands::get_session_by_id(job.session_id)
        .await
        .expect("Could not get session");

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
