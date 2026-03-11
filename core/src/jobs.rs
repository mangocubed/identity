use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct FinishedSessionJob {
    pub session_id: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct NewSessionJob {
    pub session_id: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct NewUserJob {
    pub user_id: Uuid,
}
