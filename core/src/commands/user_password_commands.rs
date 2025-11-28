use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::db_pool;
use crate::enums::ConfirmationAction;
use crate::inputs::ResetPasswordInput;
use crate::jobs_storage::jobs_storage;

use super::{encrypt_password, finish_confirmation, get_confirmation_by_id};

pub async fn reset_user_password(input: &ResetPasswordInput) -> Result<(), ValidationErrors> {
    input.validate()?;

    let confirmation =
        get_confirmation_by_id(Uuid::try_parse(&input.confirmation_id).expect("Could not parse confirmation ID"))
            .await
            .map_err(|_| ValidationErrors::new())?;

    if confirmation.action != ConfirmationAction::PasswordReset {
        return Err(ValidationErrors::new());
    }

    finish_confirmation(&confirmation.clone(), &input.confirmation_code, move || {
        let confirmation = confirmation.clone();
        async move {
            let db_pool = db_pool().await;
            let user = confirmation.user().await;

            let result = sqlx::query!(
                "UPDATE users SET encrypted_password = $2 WHERE disabled_at IS NULL AND id = $1",
                user.id,                           // $1
                encrypt_password(&input.password), // $2
            )
            .execute(db_pool)
            .await;

            match result {
                Ok(_) => {
                    jobs_storage().await.push_password_changed(&user).await;

                    Ok(())
                }
                Err(_) => Err(ValidationErrors::new()),
            }
        }
    })
    .await
}
