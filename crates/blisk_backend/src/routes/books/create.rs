use axum::{extract::State, http::StatusCode, response::Response};

use crate::{
    app::AppState,
    utils::{errors::AppError, json::AppJson, response::response},
};

#[derive(serde::Deserialize)]
pub struct CreatePayload {
    title: String,
    summary: String,
}
#[derive(serde::Serialize)]
pub struct CreateResponse {
    id: i64,
}

pub async fn create(
    State(AppState { pool, .. }): State<AppState>,
    AppJson(CreatePayload { title, summary }): AppJson<CreatePayload>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let bid: i64 = sqlx::query_scalar!(
        "INSERT INTO books (title, summary) VALUES ($1, $2) RETURNING id",
        &title,
        &summary
    )
    .fetch_one(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(response(
        StatusCode::CREATED,
        None,
        AppJson(CreateResponse { id: bid }),
    ))
}
