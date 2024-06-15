use axum::{extract::State, http::StatusCode, response::Response};
use tracing::instrument;

use crate::{
    app::AppState,
    utils::{auth::structs::UserClaims, errors::AppError, json::AppJson, response::response},
};

#[derive(serde::Deserialize)]
pub struct CreatePayload {
    title: String,
    content: String,
}
#[derive(serde::Serialize)]
pub struct CreateResponse {
    id: i64,
}

#[instrument(name = "Creating a new post", skip(pool, claims, title, content))]
pub async fn create(
    State(AppState { pool, .. }): State<AppState>,
    claims: UserClaims,
    AppJson(CreatePayload { title, content }): AppJson<CreatePayload>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let pid: i64 = sqlx::query_scalar(
        "INSERT INTO posts (author_id, title, content) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(&claims.sub)
    .bind(&title)
    .bind(&content)
    .fetch_one(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(response(
        StatusCode::CREATED,
        None,
        AppJson(CreateResponse { id: pid }),
    ))
}
