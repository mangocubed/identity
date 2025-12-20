use validator::ValidateUrl;

use sdk::core::generate_random_string;

use crate::config::APPLICATIONS_CONFIG;
use crate::db_pool;
use crate::models::Application;

use super::encrypt_password;

pub async fn all_applications<'a>() -> sqlx::Result<Vec<Application<'a>>> {
    let db_pool = db_pool().await;

    sqlx::query_as!(Application, "SELECT * FROM applications")
        .fetch_all(db_pool)
        .await
}

pub async fn delete_application(application: Application<'_>) -> sqlx::Result<()> {
    let db_pool = db_pool().await;

    sqlx::query!("DELETE FROM applications WHERE id = $1", application.id)
        .execute(db_pool)
        .await
        .map(|_| ())
}

pub async fn insert_application<'a>(
    name: &str,
    redirect_url: &str,
    webhook_url: Option<&str>,
) -> sqlx::Result<(Application<'a>, String)> {
    if name.is_empty() {
        return Err(sqlx::Error::InvalidArgument("name".to_owned()));
    }

    if !redirect_url.validate_url() {
        return Err(sqlx::Error::InvalidArgument("redirect_url".to_owned()));
    }

    if !webhook_url.validate_url() {
        return Err(sqlx::Error::InvalidArgument("webhook_url".to_owned()));
    }

    let db_pool = db_pool().await;
    let secret = generate_random_string(APPLICATIONS_CONFIG.secret_length);
    let encrypted_secret = encrypt_password(&secret);
    let webhook_secret = generate_random_string(APPLICATIONS_CONFIG.secret_length);

    sqlx::query_as!(
        Application,
        "INSERT INTO applications (name, redirect_url, encrypted_secret, webhook_url, webhook_secret)
        VALUES ($1, $2, $3, $4, $5) RETURNING *",
        name,             // $1
        redirect_url,     // $2
        encrypted_secret, // $3
        webhook_url,      // $4
        webhook_secret    // $5
    )
    .fetch_one(db_pool)
    .await
    .map(|application| (application, secret))
}

pub async fn update_application<'a>(
    application: &Application<'a>,
    redirect_url: &str,
    webhook_url: Option<&str>,
) -> sqlx::Result<Application<'a>> {
    if !redirect_url.validate_url() {
        return Err(sqlx::Error::InvalidArgument("redirect_url".to_owned()));
    }

    if !webhook_url.validate_url() {
        return Err(sqlx::Error::InvalidArgument("webhook_url".to_owned()));
    }

    if redirect_url == application.redirect_url && webhook_url == application.webhook_url.as_deref() {
        return Ok(application.clone());
    }

    let db_pool = db_pool().await;

    sqlx::query_as!(
        Application,
        "UPDATE applications SET redirect_url = $2, webhook_url = $3 WHERE id = $1 RETURNING *",
        application.id, // $1
        redirect_url,   // $2
        webhook_url,    // $3
    )
    .fetch_one(db_pool)
    .await
}

pub async fn update_application_secret<'a>(application: &Application<'_>) -> sqlx::Result<(Application<'a>, String)> {
    let db_pool = db_pool().await;

    let secret = generate_random_string(APPLICATIONS_CONFIG.secret_length);
    let encrypted_secret = encrypt_password(&secret);

    sqlx::query_as!(
        Application,
        "UPDATE applications SET encrypted_secret = $2 WHERE id = $1 RETURNING *",
        application.id,   // $1
        encrypted_secret, // $2
    )
    .fetch_one(db_pool)
    .await
    .map(|application| (application, secret))
}

pub async fn update_application_webhook_secret<'a>(application: &Application<'_>) -> sqlx::Result<Application<'a>> {
    let db_pool = db_pool().await;

    let webhook_secret = generate_random_string(APPLICATIONS_CONFIG.secret_length);

    sqlx::query_as!(
        Application,
        "UPDATE applications SET webhook_secret = $2 WHERE id = $1 RETURNING *",
        application.id, // $1
        webhook_secret, // $2
    )
    .fetch_one(db_pool)
    .await
}
