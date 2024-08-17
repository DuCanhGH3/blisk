use super::auth::OptionalUserClaims;
use super::comments::Comment;
use crate::routes::posts::Post;
use crate::utils::errors::AppError;
use crate::{
    app::AppState,
    utils::{response::response, structs::AppJson},
};
use axum::extract::Query;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("user {0} cannot be found")]
    UserNotFound(String),
    #[error("this error is not expected")]
    Unexpected,
}

#[derive(serde::Deserialize)]
pub struct ReadPostsQuery {
    offset: Option<i64>,
}

pub async fn read_posts(
    State(AppState { pool, .. }): State<AppState>,
    OptionalUserClaims(claims): OptionalUserClaims,
    Path(username): Path<String>,
    Query(ReadPostsQuery { offset }): Query<ReadPostsQuery>,
) -> Result<Response, AppError> {
    let uid = claims.as_ref().map(|claims| claims.sub);
    let offset = offset.unwrap_or(0);
    let mut transaction = pool.begin().await?;
    let posts = sqlx::query_as!(
        Post,
        r#"SELECT
            id AS "id!: _",
            title AS "title!: _",
            content AS "content!: _",
            author_name AS "author_name!: _",
            reaction AS "reaction!: _",
            user_reaction AS "user_reaction?: _",
            NULL AS "comments?: _"
        FROM fetch_posts(request_uid => $2)
        WHERE author_name = $1
        ORDER BY id DESC
        LIMIT 5
        OFFSET $3"#,
        &username,
        &uid as &_,
        &offset,
    )
    .fetch_all(&mut *transaction)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => UserError::UserNotFound(username),
        _ => UserError::Unexpected,
    })?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(posts)))
}

#[derive(serde::Deserialize)]
pub struct ReadCommentsQuery {
    previous_last: Option<i64>,
}

pub async fn read_comments(
    State(AppState { pool, .. }): State<AppState>,
    OptionalUserClaims(claims): OptionalUserClaims,
    Path(username): Path<String>,
    Query(ReadCommentsQuery { previous_last }): Query<ReadCommentsQuery>,
) -> Result<Response, AppError> {
    let uid = claims.as_ref().map(|claims| claims.sub);
    let mut transaction = pool.begin().await?;
    let comments = sqlx::query_as!(
        Comment,
        r#"SELECT
            c.id AS "id!",
            c.post_id AS "post_id!",
            c.content AS "content!",
            c.author_name AS "author_name!",
            c.user_reaction AS "user_reaction?: _",
            c.children AS "children?: _"
        FROM fetch_comments(
            request_uid => $2,
            replies_depth => 0
        ) c
        WHERE c.author_name = $1 AND CASE
            WHEN $3::BIGINT IS NULL THEN TRUE
            WHEN $3::BIGINT IS NOT NULL AND c.id < $3::BIGINT THEN TRUE
            ELSE FALSE
        END
        ORDER BY c.id DESC
        LIMIT 20"#,
        &username,
        &uid as &_,
        &previous_last as &_,
    )
    .fetch_all(&mut *transaction)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => UserError::UserNotFound(username),
        _ => UserError::Unexpected,
    })?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(comments)))
}
