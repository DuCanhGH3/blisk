use axum::{extract::State, http::StatusCode, response::Response};
use tracing::instrument;

use crate::{
    app::AppState,
    utils::{
        auth::structs::UserClaims,
        errors::AppError,
        json::AppJson,
        response::{response, SuccessResponse},
    },
};

#[derive(serde::Deserialize)]
pub struct CreatePayload {
    post_id: i64,
    parent_id: Option<i64>,
    content: String,
}

#[instrument(name = "Creating a comment", skip(pool, claims), fields(uid = %claims.sub))]
pub async fn create(
    State(AppState { pool, .. }): State<AppState>,
    claims: UserClaims,
    AppJson(CreatePayload {
        post_id,
        parent_id,
        content,
    }): AppJson<CreatePayload>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let query = {
        if let Some(parent) = parent_id {
            sqlx::query!(
                "INSERT INTO comments (post_id, author_id, content, path)
                VALUES ($1, $2, $3, (SELECT path || text2ltree(id::VARCHAR(255)) FROM comments WHERE id = $4))",
                &post_id,
                &claims.sub,
                &content,
                &parent,
            )
        } else {
            sqlx::query!(
                "INSERT INTO comments (post_id, author_id, content, path) VALUES ($1, $2, $3, 'Top')",
                &post_id,
                &claims.sub,
                &content,
            )
        }
    };
    query.execute(&mut *transaction).await?;
    transaction.commit().await?;
    Ok(response(
        StatusCode::CREATED,
        None,
        AppJson(SuccessResponse {
            message: "Comment created successfully!".to_owned(),
        }),
    ))
}
