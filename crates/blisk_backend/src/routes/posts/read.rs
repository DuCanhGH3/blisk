use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Response,
};
use tracing::instrument;

use crate::{
    app::AppState,
    utils::{
        comments::{self, structs::Comment},
        errors::AppError,
        json::AppJson,
        posts::{
            errors::PostsError,
            structs::{Post, Reaction},
        },
        response::response,
    },
};

#[derive(serde::Deserialize)]
pub struct ReadQuery {
    post_id: i64,
}
#[derive(serde::Serialize)]
pub struct ReadResponse {
    post: Post,
    comments: Vec<Comment>,
}

#[instrument(name = "Reading a post", skip(pool))]
pub async fn read(
    State(AppState { pool, .. }): State<AppState>,
    Query(ReadQuery { post_id }): Query<ReadQuery>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let post = sqlx::query_as!(
        Post,
        r#"SELECT
            p.title,
            p.content,
            u.name as author_name,
            p.reaction AS "reaction!: Reaction"
        FROM posts p
        JOIN users u ON p.author_id = u.id
        WHERE p.id = $1"#,
        &post_id,
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => PostsError::PostNotFound(post_id),
        _ => PostsError::Unexpected,
    })?;
    let comments = comments::utils::read(&mut transaction, post_id, 20, 0).await?;
    transaction.commit().await?;
    Ok(response(
        StatusCode::OK,
        None,
        AppJson(ReadResponse { post, comments }),
    ))
}
