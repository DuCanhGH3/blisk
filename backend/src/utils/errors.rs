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
        };
        (status, Json(ErrorResponse { error })).into_response()
    }
}
