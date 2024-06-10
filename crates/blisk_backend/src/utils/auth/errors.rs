use axum::{http::StatusCode, response::IntoResponse};

use crate::utils::{json::AppJson, response::ErrorResponse};

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("this user either doesn't exist or has already been verified")]
    AlreadyVerified,
    #[error("this user is not valid")]
    Invalid,
    #[error("this error is not expected")]
    Unexpected,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, error) = match self {
            AuthError::AlreadyVerified => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "The requested user is invalid or already verified.",
            ),
            AuthError::Invalid => (
                StatusCode::UNAUTHORIZED,
                "Invalid credentials provided. The requested user may or may not already exist, or may be incorrect.",
            ),
            AuthError::Unexpected => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            ),
        };
        (status, AppJson(ErrorResponse { error: error.to_owned() })).into_response()
    }
}