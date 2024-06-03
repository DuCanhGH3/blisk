use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use super::response::ErrorResponse;

pub enum ApplicationError {
    TokenUsed,
    JsonRejection(JsonRejection),
    RedisError(redis::RedisError),
    JwtError(jsonwebtoken::errors::Error),
    SqlxError(sqlx::Error),
    MiniJinjaError(minijinja::Error),
    LettreError(lettre::error::Error),
    LettreSmtpError(lettre::transport::smtp::Error),
    Argon2HashError(argon2::password_hash::Error),
}

impl From<redis::RedisError> for ApplicationError {
    fn from(value: redis::RedisError) -> Self {
        ApplicationError::RedisError(value)
    }
}
impl From<jsonwebtoken::errors::Error> for ApplicationError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        ApplicationError::JwtError(value)
    }
}
impl From<sqlx::Error> for ApplicationError {
    fn from(value: sqlx::Error) -> Self {
        ApplicationError::SqlxError(value)
    }
}
impl From<minijinja::Error> for ApplicationError {
    fn from(value: minijinja::Error) -> Self {
        ApplicationError::MiniJinjaError(value)
    }
}
impl From<lettre::error::Error> for ApplicationError {
    fn from(value: lettre::error::Error) -> Self {
        ApplicationError::LettreError(value)
    }
}
impl From<lettre::transport::smtp::Error> for ApplicationError {
    fn from(value: lettre::transport::smtp::Error) -> Self {
        ApplicationError::LettreSmtpError(value)
    }
}
impl From<argon2::password_hash::Error> for ApplicationError {
    fn from(value: argon2::password_hash::Error) -> Self {
        ApplicationError::Argon2HashError(value)
    }
}

/// TODO: more user-friendly error messages
impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            ApplicationError::TokenUsed => (
                StatusCode::BAD_REQUEST,
                "Token has either expired or been used".to_owned(),
            ),
            ApplicationError::JsonRejection(rejection) => {
                (rejection.status(), rejection.body_text())
            }
            ApplicationError::RedisError(error) => {
                tracing::error!(%error, "encountered an error from Redis");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            }
            ApplicationError::JwtError(error) => {
                tracing::error!(%error, "encountered an error from jsonwebtoken");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            }
            ApplicationError::SqlxError(error) => {
                tracing::error!(%error, "encountered an error from SQLx");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            }
            ApplicationError::MiniJinjaError(error) => {
                tracing::error!(%error, "encountered an error from MiniJinja");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            }
            ApplicationError::LettreError(error) => {
                tracing::error!(%error, "encountered an error from Lettre");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            }
            ApplicationError::LettreSmtpError(error) => {
                tracing::error!(%error, "encountered an error from Lettre");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            }
            ApplicationError::Argon2HashError(error) => {
                tracing::error!(%error, "encountered an error from Argon2");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            }
        };
        (status, Json(ErrorResponse { error })).into_response()
    }
}
