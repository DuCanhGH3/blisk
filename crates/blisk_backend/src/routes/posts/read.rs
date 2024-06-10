use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Response,
    Json,
};
use sqlx::{postgres::PgRow, Row};
use tracing::instrument;

use crate::{
    app::ApplicationState,
    utils::{errors::ApplicationError, response::response},
};

#[derive(serde::Deserialize)]
pub struct ReadQuery {
    post_id: String,
}

#[derive(serde::Serialize)]
struct Post {
    title: String,
    content: String,
}

#[instrument(name = "read", skip(pool))]
pub async fn read(
    State(ApplicationState { pool, .. }): State<ApplicationState>,
    Query(ReadQuery { post_id }): Query<ReadQuery>,
) -> Result<Response, ApplicationError> {
    let mut transaction = pool.begin().await?;
    let rows = sqlx::query("SELECT title, content FROM posts WHERE user_id = $1")
        .bind(&post_id)
        .map(|e: PgRow| Post {
            title: e.get("title"),
            content: e.get("content"),
        })
        .fetch_all(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, Json(rows)))
}
