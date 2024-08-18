use std::convert::Infallible;

use super::{response::ErrorResponse, structs::AppJson, uploads::UploadsError};
use crate::{
    routes::{
        auth::AuthError, books::BooksError, comments::CommentsError, posts::PostsError,
        users::UserError,
    },
    utils::response::ValidationErrorResponse,
};
use axum::{
    extract::rejection::{FormRejection, JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::QueryRejection;
use axum_typed_multipart::TypedMultipartError;
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("a validator error was met: {0}")]
    ValidationErrors(#[from] ValidationErrors),
    #[error("error while authenticating an user: {0}")]
    AuthError(#[from] AuthError),
    #[error("error while processing an user: {0}")]
    UserError(#[from] UserError),
    #[error("error while processing a comment: {0}")]
    CommentsError(#[from] CommentsError),
    #[error("error while processing a post: {0}")]
    PostsError(#[from] PostsError),
    #[error("error while processing a book: {0}")]
    BooksError(#[from] BooksError),
    #[error("error while uploading a file: {0}")]
    UploadsError(#[from] UploadsError),
    #[error("error was not expected {0}")]
    Unexpected(&'static str),
    #[error("error while procesing form: {0}")]
    FormRejection(#[from] FormRejection),
    #[error("error while extracing query: {0}")]
    QueryRejection(#[from] QueryRejection),
    #[error("error while procesing json: {0}")]
    JsonRejection(#[from] JsonRejection),
    #[error("error while processing a multipart request: {0}")]
    MultipartError(#[from] TypedMultipartError),
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
    #[error("an infallible error occurred?: {0}")]
    Infallible(#[from] Infallible),
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
            AppError::ValidationErrors(error) => {
                return (StatusCode::BAD_REQUEST, AppJson(ValidationErrorResponse { validation_error: error })).into_response();
            },
            AppError::AuthError(error) => {
                match error {
                    AuthError::AlreadyVerified => (
                        StatusCode::UNPROCESSABLE_ENTITY,
                        "The requested user is invalid or already verified.".to_owned(),
                    ),
                    AuthError::TokenUsed => (
                        StatusCode::UNPROCESSABLE_ENTITY,
                        "Token has either expired or been used.".to_owned(),
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
            AppError::UserError(error) => {
                match error {
                    UserError::UserNotFound(username) => (
                        StatusCode::NOT_FOUND,
                        format!("User {} may have been banned, or the username is incorrect.", username)
                    ),
                    UserError::Unexpected => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error".to_owned()
                    )
                }
            }
            AppError::CommentsError(error) => {
                match error {
                    CommentsError::CommentNotFound(_) => {
                        (StatusCode::NOT_FOUND, "Post not found.".to_owned())
                    }
                    CommentsError::UpdateUnauthorized(id) => {
                        (StatusCode::UNAUTHORIZED, format!("Comment {id} is either not yours or not found."))
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
                        (StatusCode::NOT_FOUND, "Post not found.".to_owned())
                    }
                    PostsError::UpdateUnauthorized(id) => {
                        (StatusCode::UNAUTHORIZED, format!("Post {id} is either not yours or not found."))
                    }
                    PostsError::Unexpected => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error".to_owned(),
                    ),
                }
            }
            AppError::BooksError(error) => {
                match error {
                    BooksError::SlugAlreadyExists(slug) => (
                        StatusCode::CONFLICT,
                        format!("Slug {slug} already exists!")
                    ),
                    BooksError::LanguageInvalid(lang) => (
                        StatusCode::CONFLICT,
                        format!("Language {lang} is not valid.")
                    ),
                    BooksError::BookNotFound(_) => (
                        StatusCode::NOT_FOUND,
                        format!("Book not found.")
                    ),
                    BooksError::Unexpected => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error".to_owned()
                    )
                }
            }
            AppError::UploadsError(error) => {
                match error {
                    UploadsError::InvalidName(file) => (
                        StatusCode::BAD_REQUEST,
                        format!("{file} is not a valid filename!")
                    ),
                    UploadsError::IoError(_) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error".to_owned()
                    ),
                    UploadsError::Unexpected => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal Server Error".to_owned()
                    )
                }
            },
            AppError::Unexpected(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),
            ),
            AppError::FormRejection(rejection) => (rejection.status(), rejection.body_text()),
            AppError::QueryRejection(rejection) => match rejection {
                QueryRejection::FailedToDeserializeQueryString(inner) => (
                    StatusCode::BAD_REQUEST,
                    format!("Failed to deserialize query string: {inner}")
                ),
                _ => (
                    StatusCode::BAD_REQUEST,
                    "Failed to deserialize query string".to_owned()
                )
            },
            AppError::JsonRejection(rejection) => (rejection.status(), rejection.body_text()),
            AppError::MultipartError(error) => (error.get_status(), error.to_string()),
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
            AppError::Infallible(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_owned(),                
            )
        };
        (status, AppJson(ErrorResponse { error })).into_response()
    }
}
