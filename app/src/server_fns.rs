use dioxus::prelude::*;

use sdk::serv_fn::{FormResult, ServFnClient};

#[cfg(feature = "server")]
use sdk::serv_fn::{FormError, FormSuccess, ServFnError, ServFnResult, extract_header_value};

use identity_core::inputs::RegisterInput;

#[cfg(feature = "server")]
async fn extract_app_token() -> ServFnResult<Option<String>> {
    let Some(app_token) = extract_header_value(&crate::constants::HEADER_APP_TOKEN).await? else {
        return Ok(None);
    };

    Ok(app_token.to_str().ok().map(|s| s.to_owned()))
}

#[cfg(feature = "server")]
async fn require_app_token() -> ServFnResult<()> {
    let app_token = extract_app_token().await?;

    if let Some(app_token) = app_token
        && identity_core::commands::verify_app_token(&app_token)
    {
        Ok(())
    } else {
        Err(ServFnError::forbidden().into())
    }
}

#[server(client = ServFnClient)]
pub async fn attempt_to_register(input: RegisterInput) -> FormResult {
    require_app_token().await.map_err(|error| FormError::from(error))?;

    let result = identity_core::commands::insert_user(&input).await;

    match result {
        Ok(_) => Ok(FormSuccess::new("User created successfully")),
        Err(errors) => Err(FormError::new("Failed to create user", Some(errors)).into()),
    }
}
