use cached::AsyncRedisCache;
use cached::proc_macro::io_cached;
use uuid::Uuid;
use validator::Validate;

use toolbox::cache::{AsyncRedisCacheExt, redis_cache_store};
use toolbox::validator::{OrValidationErrors, ValidationResult};

use crate::constants::CACHE_PREFIX_GET_APPLICATION_BY_ID;
use crate::db_pool;
use crate::models::Application;
use crate::params::ApplicationParams;

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
        .await?;

    remove_application_cache(&application).await;

    Ok(())
}

#[io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    ty = "AsyncRedisCache<Uuid, Application<'_>>",
    create = r##"{ redis_cache_store(CACHE_PREFIX_GET_APPLICATION_BY_ID).await }"##
)]
pub async fn get_application_by_id(id: Uuid) -> sqlx::Result<Application<'static>> {
    let db_pool = db_pool().await;

    sqlx::query_as!(Application, "SELECT * FROM applications WHERE id = $1 LIMIT 1", id)
        .fetch_one(db_pool)
        .await
}

pub async fn insert_application<'a>(params: ApplicationParams) -> ValidationResult<Application<'a>> {
    params.validate()?;

    let db_pool = db_pool().await;

    let application = sqlx::query_as!(
        Application,
        "INSERT INTO applications (name, redirect_url) VALUES ($1, $2) RETURNING *",
        params.name,         // $1
        params.redirect_url, // $2
    )
    .fetch_one(db_pool)
    .await
    .or_validation_errors()?;

    Ok(application)
}

pub async fn remove_application_cache(application: &Application<'_>) {
    GET_APPLICATION_BY_ID
        .cache_remove(CACHE_PREFIX_GET_APPLICATION_BY_ID, &application.id)
        .await;
}

pub async fn update_application<'a>(
    application: &Application<'a>,
    params: ApplicationParams,
) -> ValidationResult<Application<'a>> {
    params.validate()?;

    let db_pool = db_pool().await;

    let application = sqlx::query_as!(
        Application,
        "UPDATE applications SET name = $2, redirect_url = $3 WHERE id = $1 RETURNING *",
        application.id,      // $1
        params.name,         // $2
        params.redirect_url, // $3
    )
    .fetch_one(db_pool)
    .await
    .or_validation_errors()?;

    remove_application_cache(&application).await;

    Ok(application)
}
