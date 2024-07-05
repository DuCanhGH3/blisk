use axum::{extract::State, http::StatusCode, response::Response};
use axum_typed_multipart::TryFromMultipart;

use crate::{
    app::AppState,
    utils::{
        errors::AppError,
        response::response,
        structs::{AppJson, AppMultipart},
    },
};

#[derive(TryFromMultipart)]
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
    AppMultipart(CreatePayload { title, summary }): AppMultipart<CreatePayload>,
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
