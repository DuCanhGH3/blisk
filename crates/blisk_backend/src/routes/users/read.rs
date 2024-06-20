use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Response,
};

use crate::{
    app::AppState,
    utils::{errors::AppError, json::AppJson, response::response, users::errors::UserError},
};

#[derive(serde::Deserialize)]
pub struct ReadQuery {
    username: String,
}
#[derive(serde::Serialize, serde::Deserialize)]
struct ReadResponseComment {
    content: String,
}
#[derive(serde::Serialize, serde::Deserialize)]
struct ReadResponsePost {
    title: String,
    content: String,
    comments: Vec<ReadResponseComment>,
}
#[derive(serde::Serialize)]
struct ReadResponse {
    name: String,
    posts: sqlx::types::Json<Vec<ReadResponsePost>>,
}

pub async fn read(
    State(AppState { pool, .. }): State<AppState>,
    Query(ReadQuery { username }): Query<ReadQuery>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let user = sqlx::query_as!(
        ReadResponse,
        r#"SELECT
            u.name,
            COALESCE(JSON_AGG(p) FILTER (WHERE p IS NOT NULL), '[]'::JSON) as "posts!: sqlx::types::Json<Vec<ReadResponsePost>>"
        FROM users u
        LEFT JOIN LATERAL (
            SELECT
                p.title,
                p.content,
                COALESCE(JSON_AGG(c) FILTER (WHERE c IS NOT NULL), '[]'::JSON) as comments
            FROM posts p
            LEFT JOIN LATERAL (
                SELECT c.content
                FROM comments c
                WHERE c.post_id = p.id AND c.author_id = u.id
                ORDER BY c.id DESC
                LIMIT 5
            ) c ON TRUE
            WHERE p.author_id = u.id
            GROUP BY p.id
            ORDER BY p.id DESC
            LIMIT 5
        ) p ON TRUE
        WHERE u.name = $1
        GROUP BY u.id"#,
        &username
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => UserError::UserNotFound(username),
        _ => UserError::Unexpected,
    })?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(user)))
}
