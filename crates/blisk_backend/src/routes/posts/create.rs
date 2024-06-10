use axum::{extract::State, http::StatusCode, response::Response, Json};
use tracing::instrument;

use crate::{
    app::ApplicationState,
    utils::{auth::structs::UserClaims, errors::ApplicationError, response::response},
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
    State(ApplicationState { pool, .. }): State<ApplicationState>,
    claims: UserClaims,
    Json(CreatePayload { title, content }): Json<CreatePayload>,
) -> Result<Response, ApplicationError> {
    let mut transaction = pool.begin().await?;
    let pid: i64 = sqlx::query_scalar(
        "INSERT INTO posts (user_id, title, content) VALUES ($1, $2, $3) RETURNING id",
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
        Json(CreateResponse { id: pid }),
    ))
}
