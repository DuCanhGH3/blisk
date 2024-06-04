use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::utils::response::ErrorResponse;

pub enum AuthError {
    AlreadyVerified,
    Invalid,
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
        (status, Json(ErrorResponse { error: error.to_owned() })).into_response()
    }
}