use axum::{extract::State, http::StatusCode, response::Response};
use tracing::instrument;

use crate::{
    app::AppState,
    utils::{
        users::structs::UserClaims, errors::AppError, json::AppJson, posts::structs::Reaction,
        response::response,
    },
};

#[derive(serde::Deserialize)]
pub struct CreatePayload {
    book_id: i64,
    title: String,
    content: String,
    reaction: Reaction,
}
#[derive(serde::Serialize)]
pub struct CreateResponse {
    id: i64,
}

#[instrument(name = "Creating a new post", skip(pool, claims, title, content))]
pub async fn create(
    State(AppState { pool, .. }): State<AppState>,
    claims: UserClaims,
    AppJson(CreatePayload {
        book_id,
        title,
        content,
        reaction,
    }): AppJson<CreatePayload>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let pid: i64 = sqlx::query_scalar!(
        "INSERT INTO posts (author_id, book_id, title, content, reaction) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        &claims.sub,
        &book_id,
        &title,
        &content,
        &reaction as _
    )
    .fetch_one(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(response(
        StatusCode::CREATED,
        None,
        AppJson(CreateResponse { id: pid }),
    ))
}
