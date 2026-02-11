use apalis::prelude::BoxDynError;

use identity_core::commands;
use identity_core::jobs::{NewUserJob};

use crate::mailer::{admin_emails, send_welcome_email};

pub async fn new_user(job: NewUserJob) -> Result<(), BoxDynError> {
    let user = commands::get_user_by_id(job.user_id).await?;

    let _ = admin_emails::send_new_user_email(&user).await;

    send_welcome_email(&user).await
}
