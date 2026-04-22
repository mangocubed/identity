use uuid::Uuid;
use validator::ValidationErrors;

use toolbox::constants::ERROR_IS_INVALID;
use toolbox::validator::ValidationResult;

use crate::config::CONFIRMATION_CONFIG;
use crate::enums::ConfirmationAction;
use crate::models::{Confirmation, User};
use crate::{db_pool, jobs_storage};

use super::{encrypt_password, generate_random_string};

pub async fn finish_confirmation<F, IF, T>(
    confirmation: &Confirmation<'_>,
    code: &str,
    on_success: F,
) -> ValidationResult<T>
where
    F: Fn() -> IF,
    IF: std::future::IntoFuture<Output = ValidationResult<T>>,
{
    let db_pool = db_pool().await;

    if !confirmation.verify_code(code) {
        let _ = sqlx::query!(
            "UPDATE confirmations
            SET
                canceled_at = CASE pending_attempts WHEN 1 THEN current_timestamp ELSE NULL END,
                pending_attempts = pending_attempts - 1
            WHERE id = $1",
            confirmation.id
        )
        .execute(db_pool)
        .await;

        let mut validation_errors = ValidationErrors::new();

        validation_errors.add("confirmation_code", ERROR_IS_INVALID.clone());

        return Err(validation_errors);
    }

    let result = on_success().await;

    match result {
        Ok(success) => {
            let _ = sqlx::query!(
                "UPDATE confirmations SET finished_at = current_timestamp
                WHERE finished_at IS NULL AND canceled_at IS NULL AND id = $1",
                confirmation.id
            )
            .execute(db_pool)
            .await;

            Ok(success)
        }
        Err(errors) => Err(errors),
    }
}

pub async fn get_confirmation_by_id<'a>(id: Uuid) -> sqlx::Result<Confirmation<'a>> {
    let db_pool = db_pool().await;

    sqlx::query_as!(
        Confirmation,
        r#"SELECT
            id,
            user_id,
            action as "action!: ConfirmationAction",
            encrypted_code,
            pending_attempts,
            created_at,
            updated_at
        FROM confirmations
        WHERE
            id = $1 AND pending_attempts > 0 AND expires_at > current_timestamp AND finished_at IS NULL
            AND canceled_at IS NULL
        LIMIT 1"#,
        id
    )
    .fetch_one(db_pool)
    .await
}

pub async fn get_confirmation_by_user<'a>(
    user: &User<'_>,
    action: ConfirmationAction,
) -> sqlx::Result<Confirmation<'a>> {
    let db_pool = db_pool().await;

    sqlx::query_as!(
        Confirmation,
        r#"SELECT
            id,
            user_id,
            action as "action!: ConfirmationAction",
            encrypted_code,
            pending_attempts,
            created_at,
            updated_at
        FROM confirmations
        WHERE
            user_id = $1 AND action = $2 AND pending_attempts > 0 AND expires_at > current_timestamp
            AND finished_at IS NULL AND canceled_at IS NULL
        LIMIT 1"#,
        user.id,                      // $1
        action as ConfirmationAction, // $2
    )
    .fetch_one(db_pool)
    .await
}

pub async fn insert_confirmation<'a>(user: &User<'_>, action: ConfirmationAction) -> sqlx::Result<Confirmation<'a>> {
    let db_pool = db_pool().await;

    let _ = sqlx::query!(
        "UPDATE confirmations SET canceled_at = current_timestamp
        WHERE user_id = $1 AND action = $2 AND finished_at IS NULL AND canceled_at IS NULL",
        user.id,                      // $1
        action as ConfirmationAction, // $2
    )
    .execute(db_pool)
    .await;

    let code = generate_random_string(CONFIRMATION_CONFIG.code_length..=CONFIRMATION_CONFIG.code_length);

    let encrypted_code = encrypt_password(&code);

    let result = sqlx::query_as!(
        Confirmation,
        r#"INSERT INTO confirmations (user_id, action, encrypted_code) VALUES ($1, $2, $3)
            RETURNING
                id,
                user_id,
                action as "action!: ConfirmationAction",
                encrypted_code,
                pending_attempts,
                created_at,
                updated_at"#,
        user.id,                      // $1
        action as ConfirmationAction, // $2
        encrypted_code                // $3
    )
    .fetch_one(db_pool)
    .await;

    match result {
        Ok(confirmation) => {
            jobs_storage().await.push_new_confirmation(&confirmation, &code).await;

            Ok(confirmation)
        }
        Err(error) => Err(error),
    }
}
