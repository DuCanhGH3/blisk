use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};

use crate::{
    app::AppState,
    utils::{
        errors::AppError,
        response::response,
        structs::{AppImage, AppJson},
    },
};

use super::books::Book;

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("user {0} cannot be found")]
    UserNotFound(String),
    #[error("this error is not expected")]
    Unexpected,
}

#[derive(serde::Serialize)]
pub struct UserMetadata {
    pub name: String,
    pub picture: Option<sqlx::types::Json<AppImage>>,
    pub books: sqlx::types::Json<Vec<Book>>,
}

pub async fn read_metadata(
    State(AppState { pool, .. }): State<AppState>,
    Path(user): Path<String>,
) -> Result<Response, AppError> {
    let mut tx = pool.begin().await?;
    let metadata = sqlx::query_as!(
        UserMetadata,
        r#"SELECT
            u.name AS "name!",
            u.picture AS "picture!: _",
            coalesce(jsonb_agg(b) FILTER (WHERE b.id IS NOT NULL), '[]'::JSONB) AS "books!: _"
        FROM users_view u
        LEFT JOIN LATERAL (
            SELECT
                b.id,
                b.title,
                b.name,
                b.summary,
                b.cover_image,
                b.spine_image,
                b.authors,
                b.categories
            FROM users_books ub
            JOIN books_view b
            ON ub.book_id = b.id
            WHERE ub.user_id = u.id AND ub.completed = TRUE
            ORDER BY ub.ends_at DESC
        ) b ON TRUE
        WHERE u.name = $1
        GROUP BY u.name, u.picture"#,
        &user,
    )
    .fetch_one(&mut *tx)
    .await?;
    Ok(response(StatusCode::OK, None, AppJson(metadata)))
}
