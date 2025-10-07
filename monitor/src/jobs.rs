use std::borrow::Cow;

use serde::Deserialize;

use identity_core::commands::{get_session_by_id, get_user_by_id, update_session_location};
use identity_core::config::IP_GEOLOCATION_CONFIG;
use identity_core::jobs_storage::{NewSession, NewUser};

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

pub async fn new_user_job(job: NewUser) -> Result<(), apalis::prelude::Error> {
    let user = get_user_by_id(job.user_id).await.expect("Could not get user");

    send_new_user_email(&user).await
}

pub async fn new_session_job(job: NewSession) -> Result<(), apalis::prelude::Error> {
    let session = get_session_by_id(job.session_id).await.expect("Could not get session");

    let ip_geo: IpGeo = reqwest::get(format!(
        "https://api.ipgeolocation.io/v2/ipgeo?apiKey={}&ip={}",
        IP_GEOLOCATION_CONFIG.api_key, job.ip_addr
    ))
    .await
    .expect("Could not get location")
    .json()
    .await
    .expect("Could not parse location");

    let result = update_session_location(
        &session,
        &ip_geo.location.country_code2,
        &ip_geo.location.state_prov,
        &ip_geo.location.city,
    )
    .await;

    send_new_session_email(&result.unwrap_or(session)).await
}
