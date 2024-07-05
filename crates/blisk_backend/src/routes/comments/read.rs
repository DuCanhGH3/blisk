use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Response,
};
use tracing::instrument;

use crate::{
    app::AppState,
    utils::{comments, errors::AppError, response::response, structs::AppJson},
};

#[derive(serde::Deserialize)]
pub struct ReadQuery {
    post_id: i64,
    offset: i64,
}

#[instrument(name = "Reading a comment", skip(pool))]
pub async fn read(
    State(AppState { pool, .. }): State<AppState>,
    Query(ReadQuery { post_id, offset }): Query<ReadQuery>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let comments_list = comments::utils::read(&mut transaction, post_id, 20, offset).await?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(comments_list)))
}
