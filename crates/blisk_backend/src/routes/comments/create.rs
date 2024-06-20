use axum::{extract::State, http::StatusCode, response::Response};
use tracing::instrument;

use crate::{
    app::AppState,
    utils::{errors::AppError, json::AppJson, response::response, users::structs::UserClaims},
};

#[derive(serde::Deserialize)]
pub struct CreatePayload {
    post_id: i64,
    parent_id: Option<i64>,
    content: String,
}
#[derive(serde::Serialize)]
pub struct CreateResponse {
    id: i64,
}

#[instrument(name = "Creating a comment", skip(pool, claims), fields(uid = %claims.sub))]
pub async fn create(
    State(AppState { pool, .. }): State<AppState>,
    claims: UserClaims,
    AppJson(CreatePayload { post_id, parent_id, content }): AppJson<CreatePayload>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let query = {
        if let Some(parent) = parent_id {
            sqlx::query_scalar!(
                "INSERT INTO comments (post_id, author_id, content, path)
                VALUES ($1, $2, $3, (SELECT path || TEXT2LTREE(id::VARCHAR(255)) FROM comments WHERE id = $4))
                RETURNING id",
                &post_id,
                &claims.sub,
                &content,
                &parent,
            )
        } else {
            sqlx::query_scalar!(
                "INSERT INTO comments (post_id, author_id, content, path) VALUES ($1, $2, $3, 'Top') RETURNING id",
                &post_id,
                &claims.sub,
                &content,
            )
        }
    };
    let post_id = query.fetch_one(&mut *transaction).await?;
    transaction.commit().await?;
    Ok(response(
        StatusCode::CREATED,
        None,
        AppJson(CreateResponse { id: post_id }),
    ))
}
