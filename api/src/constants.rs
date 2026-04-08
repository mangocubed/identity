use http::StatusCode;

pub const ERROR_BAD_REQUEST: (StatusCode, &str) = (StatusCode::BAD_REQUEST, "\"Bad Request\"");
#[allow(dead_code)]
pub const ERROR_FORBIDDEN: (StatusCode, &str) = (StatusCode::FORBIDDEN, "\"Forbidden\"");
pub const ERROR_INTERNAL_SERVER_ERROR: (StatusCode, &str) =
    (StatusCode::INTERNAL_SERVER_ERROR, "\"Internal Server Error\"");
pub const ERROR_NOT_FOUND: (StatusCode, &str) = (StatusCode::NOT_FOUND, "\"Not Found\"");
pub const ERROR_UNAUTHORIZED: (StatusCode, &str) = (StatusCode::UNAUTHORIZED, "\"Unauthorized\"");
