use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::{auth::errors::AuthError, json::AppJson, response::ErrorResponse};

pub enum AppError {
    TokenUsed,
    AuthError(AuthError),
    Unexpected(&'static str),
    JsonRejection(JsonRejection),
    RedisError(redis::RedisError),
    JwtError(jsonwebtoken::errors::Error),
    SqlxError(sqlx::Error),
    MiniJinjaError(minijinja::Error),
    LettreError(lettre::error::Error),
    LettreSmtpError(lettre::transport::smtp::Error),
    Argon2HashError(argon2::password_hash::Error),
}

impl From<AuthError> for AppError {
    fn from(value: AuthError) -> Self {
        AppError::AuthError(value)
    }
}
impl From<JsonRejection> for AppError {
    fn from(value: JsonRejection) -> Self {
        AppError::JsonRejection(value)
    }
}
impl From<redis::RedisError> for AppError {
    fn from(value: redis::RedisError) -> Self {
        AppError::RedisError(value)
    }
}
impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        AppError::JwtError(value)
    }
}
impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::SqlxError(value)
    }
}
impl From<minijinja::Error> for AppError {
    fn from(value: minijinja::Error) -> Self {
        AppError::MiniJinjaError(value)
    }
}
impl From<lettre::error::Error> for AppError {
    fn from(value: lettre::error::Error) -> Self {
        AppError::LettreError(value)
    }
}
impl From<lettre::transport::smtp::Error> for AppError {
    fn from(value: lettre::transport::smtp::Error) -> Self {
        AppError::LettreSmtpError(value)
    }
}
impl From<argon2::password_hash::Error> for AppError {
    fn from(value: argon2::password_hash::Error) -> Self {
        AppError::Argon2HashError(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            AppError::TokenUsed => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "Token has either expired or been used.".to_owned(),
            ),
            AppError::AuthError(error) => {
                tracing::error!(%error, "encountered an AuthN error");
                return error.into_response();
            },
            AppError::Unexpected(error) => {
                tracing::error!(%error, "encountered an unexpected error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            }
            AppError::JsonRejection(rejection) => {
                (rejection.status(), rejection.body_text())
            }
            AppError::RedisError(error) => {
                tracing::error!(%error, "encountered an error from Redis");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            }
            AppError::JwtError(error) => {
                tracing::error!(%error, "encountered an error from jsonwebtoken");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            }
            AppError::SqlxError(error) => {
                tracing::error!(%error, "encountered an error from SQLx");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            }
            AppError::MiniJinjaError(error) => {
                tracing::error!(%error, "encountered an error from MiniJinja");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            }
            AppError::LettreError(error) => {
                tracing::error!(%error, "encountered an error from Lettre");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            }
            AppError::LettreSmtpError(error) => {
                tracing::error!(%error, "encountered an error from Lettre");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            }
            AppError::Argon2HashError(error) => {
                tracing::error!(%error, "encountered an error from Argon2");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_owned(),
                )
            }
        };
        (status, AppJson(ErrorResponse { error })).into_response()
    }
}
