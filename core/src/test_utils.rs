pub use sdk::test_utils::{fake_birthdate, fake_country_alpha2, fake_email, fake_name, fake_password, fake_username};

use crate::commands::insert_user;
use crate::inputs::RegisterInput;
use crate::models::User;

pub async fn insert_test_user<'a>(password: Option<&str>) -> User<'a> {
    let password = if let Some(password) = password {
        password.to_owned()
    } else {
        fake_password()
    };

    let input = RegisterInput {
        username: fake_username(),
        email: fake_email(),
        password,
        full_name: fake_name(),
        birthdate: fake_birthdate().to_string(),
        country_alpha2: fake_country_alpha2(),
    };

    insert_user(&input).await.expect("Could not insert user")
}
