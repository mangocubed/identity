use std::net::IpAddr;
use std::str::FromStr;

use apalis::prelude::BoxDynError;

use identity_core::commands;
use identity_core::jobs::{FinishedSessionJob, NewConfirmationJob, NewSessionJob, NewUserJob, PasswordChangedJob};

use crate::ip_geo::IpGeo;
use crate::mailer::*;

pub async fn finished_session(job: FinishedSessionJob) -> Result<(), BoxDynError> {
    let session = commands::get_finished_session_by_id(job.session_id).await?;

    let access_tokens = session.access_tokens().await?;

    for access_token in access_tokens {
        let _ = commands::revoke_access_token(&access_token).await;
    }

    Ok(())
}

pub async fn new_confirmation(job: NewConfirmationJob) -> Result<(), BoxDynError> {
    let confirmation = commands::get_confirmation_by_id(job.confirmation_id).await?;

    send_new_confirmation_email(&confirmation, &job.code).await
}

pub async fn new_session(job: NewSessionJob) -> Result<(), BoxDynError> {
    let mut session = commands::get_session_by_id(job.session_id).await?;
    let ip_geo = IpGeo::new();

    if let Some(ref ip_address) = session.ip_address
        && let Ok(ip_addr) = IpAddr::from_str(ip_address)
        && !ip_addr.is_loopback()
        && !ip_addr.is_multicast()
        && !ip_addr.is_unspecified()
    {
        let result = ip_geo.info(ip_addr).await;

        if let Ok(ip_geo_info) = result {
            let result = commands::update_session_location(
                &session,
                &ip_geo_info.location.country_code2,
                &ip_geo_info.location.state_prov,
                &ip_geo_info.location.city,
            )
            .await;

            if let Ok(updated_session) = result {
                session = updated_session
            }
        }
    };

    send_new_session_email(&session).await
}

pub async fn new_user(job: NewUserJob) -> Result<(), BoxDynError> {
    let user = commands::get_user_by_id(job.user_id).await?;

    let _ = admin_emails::send_new_user_email(&user).await;

    send_welcome_email(&user).await
}

pub async fn password_changed(job: PasswordChangedJob) -> Result<(), BoxDynError> {
    let user = commands::get_user_by_id(job.user_id).await?;

    send_password_changed_email(&user).await
}
