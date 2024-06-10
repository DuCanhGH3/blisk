use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Response,
};
use tracing::instrument;

use crate::{
    app::AppState,
    utils::{errors::AppError, json::AppJson, response::response},
};

#[derive(serde::Deserialize)]
pub struct ReadQuery {
    post_id: i64,
}

#[derive(serde::Serialize, sqlx::FromRow)]
struct ReadResponse {
    title: String,
    content: String,
    author_name: String,
}

#[instrument(name = "Reading a post", skip(pool))]
pub async fn read(
    State(AppState { pool, .. }): State<AppState>,
    Query(ReadQuery { post_id }): Query<ReadQuery>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let post: ReadResponse = sqlx::query_as("SELECT p.title, p.content, u.name as author_name FROM posts p JOIN users u ON p.user_id = u.id WHERE p.id = $1")
        .bind(&post_id)
        .fetch_one(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(post)))
}
