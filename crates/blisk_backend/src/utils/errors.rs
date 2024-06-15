use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::{
    auth::errors::AuthError, comments::errors::CommentsError, json::AppJson,
    posts::errors::PostsError, response::ErrorResponse,
};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("error as an token was already used")]
    TokenUsed,
    #[error("error while authenticating an user: {0}")]
    AuthError(#[from] AuthError),
    #[error("error while processing a comment: {0}")]
    CommentsError(#[from] CommentsError),
    #[error("error while processing a post: {0}")]
    PostsError(#[from] PostsError),
    #[error("error was not expected {0}")]
    Unexpected(&'static str),
    #[error("error while procesing json: {0}")]
    JsonRejection(#[from] JsonRejection),
    #[error("erro while processing json: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("error while querying redis: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("error while processing a jwt: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("error while querying database: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("error while templating with minijinja: {0}")]
    MiniJinjaError(#[from] minijinja::Error),
    #[error("error while constructing an email with lettre: {0}")]
    LettreError(#[from] lettre::error::Error),
    #[error("error while sending an email with lettre: {0}")]
    LettreSmtpError(#[from] lettre::transport::smtp::Error),
    #[error("error while hashing password: {0}")]
    Argon2HashError(argon2::password_hash::Error),
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(value: argon2::password_hash::Error) -> Self {
        AppError::Argon2HashError(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!(error = %self, "an application error was thrown");
        let (status, error) = match self {
            AppError::TokenUsed => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "Token has either expired or been used.".to_owned(),
            ),
            AppError::AuthError(error) => {
                match error {
                    AuthError::AlreadyVerified => (
                        StatusCode::UNPROCESSABLE_ENTITY,
                        "The requested user is invalid or already verified.".to_owned(),
                    ),
                    AuthError::Invalid => (
                        StatusCode::UNAUTHORIZED,
                        "Invalid credentials provided. The requested user may or may not already exist, or may be incorrect.".to_owned(),
                    ),
                    AuthError::Unexpected => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error".to_owned(),
                    ),
                }
            }
            AppError::CommentsError(error) => {
                match error {
                    CommentsError::CommentNotFound(_) => {
                        (StatusCode::NOT_FOUND, "Post not found".to_owned())
                    }
                    CommentsError::Unexpected => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error".to_owned(),
                    ),
                }
            },
            AppError::PostsError(error) => {
                match error {
                    PostsError::PostNotFound(_) => {
                        (StatusCode::NOT_FOUND, "Post not found".to_owned())
                    }
                    PostsError::Unexpected => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error".to_owned(),
                    ),
                }
            }
            AppError::Unexpected(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            ),
            AppError::JsonRejection(rejection) => (rejection.status(), rejection.body_text()),
            AppError::SerdeError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            ),
            AppError::RedisError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            ),
            AppError::JwtError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            ),
            AppError::SqlxError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            ),
            AppError::MiniJinjaError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            ),
            AppError::LettreError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            ),
            AppError::LettreSmtpError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            ),
            AppError::Argon2HashError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            ),
        };
        (status, AppJson(ErrorResponse { error })).into_response()
    }
}
