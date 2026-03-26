use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct FinishedSessionJob {
    pub session_id: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct NewConfirmationJob {
    pub confirmation_id: Uuid,
    pub code: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewSessionJob {
    pub session_id: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct NewUserJob {
    pub user_id: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct PasswordChangedJob {
    pub user_id: Uuid,
}
