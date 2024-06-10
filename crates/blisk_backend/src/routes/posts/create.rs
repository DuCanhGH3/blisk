use axum::{extract::State, http::StatusCode, response::Response, Json};
use tracing::instrument;

use crate::{
    app::ApplicationState,
    utils::{
        errors::ApplicationError,
        response::{response, SuccessResponse},
    },
};

#[derive(serde::Deserialize)]
pub struct CreatePayload {
    user_id: String,
    title: String,
    content: String,
}

#[instrument(name = "Creating a new post", skip(pool, user_id, title, content))]
pub async fn create(
    State(ApplicationState { pool, .. }): State<ApplicationState>,
    Json(CreatePayload {
        user_id,
        title,
        content,
    }): Json<CreatePayload>,
) -> Result<Response, ApplicationError> {
    let mut transaction = pool.begin().await?;
    sqlx::query("INSERT INTO posts (user_id, title, content) VALUES ($1, $2, $3)")
        .bind(&user_id)
        .bind(&title)
        .bind(&content)
        .execute(&mut *transaction)
        .await?;
    transaction.commit().await?;
    Ok(response(
        StatusCode::CREATED,
        None,
        Json(SuccessResponse {
            message: "Post created successfully!".to_owned(),
        }),
    ))
}
