#[derive(sqlx::Type, Clone, Copy, PartialEq)]
#[sqlx(type_name = "confirmation_action", rename_all = "snake_case")]
pub enum ConfirmationAction {
    Email,
    Login,
    PasswordReset,
}
