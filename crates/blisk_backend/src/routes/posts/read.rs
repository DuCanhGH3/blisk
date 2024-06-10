use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Response,
    Json,
};
use tracing::instrument;

use crate::{
    app::ApplicationState,
    utils::{errors::ApplicationError, response::response},
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
    State(ApplicationState { pool, .. }): State<ApplicationState>,
    Query(ReadQuery { post_id }): Query<ReadQuery>,
) -> Result<Response, ApplicationError> {
    let mut transaction = pool.begin().await?;
    let post: ReadResponse = sqlx::query_as("SELECT p.title, p.content, u.name as author_name FROM posts p JOIN users u ON p.user_id = u.id WHERE p.id = $1")
        .bind(&post_id)
        .fetch_one(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, Json(post)))
}
