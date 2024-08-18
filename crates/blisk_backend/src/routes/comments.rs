use super::{
    auth::{OptionalUserClaims, UserClaims},
    reactions::PostReaction,
};
use crate::{
    app::AppState,
    utils::{errors::AppError, response::response, structs::AppJson},
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::instrument;
use validator::Validate;

#[derive(Debug, thiserror::Error)]
pub enum CommentsError {
    #[error("comment {0} cannot be found")]
    CommentNotFound(i64),
    #[error("comment {0} was non-existent, or an unauthorized personnel tried to update it")]
    UpdateUnauthorized(i64),
    #[error("this error is not expected")]
    Unexpected,
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Comment {
    pub id: i64,
    pub post_id: i64,
    pub content: String,
    pub author_name: String,
    #[sqlx(default)]
    pub user_reaction: Option<PostReaction>,
    #[sqlx(default)]
    pub children: Option<sqlx::types::Json<Vec<Comment>>>,
}

#[derive(serde::Deserialize, Validate)]
pub struct CreatePayload {
    #[validate(range(min = 0))]
    post_id: i64,
    #[validate(range(min = 0))]
    parent_id: Option<i64>,
    #[validate(length(min = 1))]
    content: String,
}
#[derive(serde::Serialize)]
pub struct CreateResponse {
    id: i64,
}

#[instrument(name = "Creating a comment", skip(pool, claims, content), fields(uid = %claims.sub))]
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

#[derive(serde::Deserialize)]
pub struct ReadQuery {
    /// The post that owns the comments to be read.
    post_id: i64,
    /// The comment to be fetched. If specified,
    /// we will only fetch this comment and its
    /// descendants.
    comment_id: Option<i64>,
    /// The last seen comment. If specified, we will only
    /// fetch comments with `id` lower than this value.
    /// This is because comments added earlier always
    /// have their `id`'s lower.
    previous_last: Option<i64>,
}

#[instrument(name = "Reading a comment", skip(pool, claims))]
pub async fn read(
    State(AppState { pool, .. }): State<AppState>,
    OptionalUserClaims(claims): OptionalUserClaims,
    Query(ReadQuery {
        post_id,
        comment_id,
        previous_last,
    }): Query<ReadQuery>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let uid = claims.as_ref().map(|claims| claims.sub);
    let comments = sqlx::query_as!(
        Comment,
        r#"SELECT
            c.id AS "id!",
            c.post_id AS "post_id!",
            c.content AS "content!",
            c.author_name AS "author_name!",
            c.user_reaction AS "user_reaction!: _",
            children AS "children?: _"
        FROM fetch_comments(
            request_uid => $1,
            replies_depth => 4
        ) c
        WHERE c.post_id = $2 AND CASE
            WHEN $3::BIGINT IS NULL AND $4::BIGINT IS NULL AND c.path = 'Top' THEN TRUE
            WHEN $3::BIGINT IS NULL AND $4::BIGINT IS NOT NULL AND c.path = 'Top' AND c.id < $4::BIGINT THEN TRUE
            WHEN $3::BIGINT IS NOT NULL AND c.id = $3::BIGINT THEN TRUE
            ELSE FALSE
        END
        ORDER BY c.id DESC
        LIMIT 20"#,
        &uid as &_,
        &post_id,
        &comment_id as &_,
        &previous_last as &_,
    )
    .fetch_all(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(comments)))
}

#[derive(serde::Deserialize)]
pub struct ReadRepliesQuery {
    /// The comment the replies of which are to be fetched.
    comment_id: i64,
    /// The last seen reply. If specified, we will only
    /// fetch replies with `id` lower than this value.
    /// This is because replies added earlier always
    /// have their `id`'s lower.
    previous_last: Option<i64>,
}

#[instrument(name = "Reading a comment's replies...", skip(pool, claims))]
pub async fn read_replies(
    State(AppState { pool, .. }): State<AppState>,
    OptionalUserClaims(claims): OptionalUserClaims,
    Query(ReadRepliesQuery {
        comment_id,
        previous_last,
    }): Query<ReadRepliesQuery>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let uid = claims.as_ref().map(|claims| claims.sub);
    let comments = sqlx::query_as!(
        Comment,
        r#"WITH parent AS (SELECT id, post_id, path FROM comments WHERE id = $2)
        SELECT
            c.id AS "id!",
            c.post_id AS "post_id!",
            c.content AS "content!",
            c.author_name AS "author_name!",
            c.user_reaction AS "user_reaction!: _",
            children AS "children?: _"
        FROM fetch_comments(
            request_uid => $1,
            replies_depth => 3
        ) c, parent
        WHERE c.post_id = parent.post_id AND c.path = parent.path || TEXT2LTREE(parent.id::TEXT) AND CASE
            WHEN $3::BIGINT IS NULL THEN TRUE
            WHEN $3::BIGINT IS NOT NULL AND c.id < $3::BIGINT THEN TRUE
            ELSE FALSE
        END
        ORDER BY c.id DESC
        LIMIT 20"#,
        &uid as &_,
        &comment_id as &_,
        &previous_last as &_,
    )
    .fetch_all(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(comments)))
}

#[derive(serde::Deserialize, Validate)]
pub struct UpdatePayload {
    #[validate(range(min = 0))]
    id: i64,
    #[validate(length(min = 1))]
    content: String,
}

#[instrument(name = "Updating a comment", skip(pool, claims), fields(uid = %claims.sub))]
pub async fn update(
    State(AppState { pool, .. }): State<AppState>,
    claims: UserClaims,
    AppJson(UpdatePayload { id, content }): AppJson<UpdatePayload>,
) -> Result<impl IntoResponse, AppError> {
    let mut transaction = pool.begin().await?;
    let update_result = sqlx::query!(
        "UPDATE comments SET content = $2 WHERE id = $1 AND author_id = $3",
        &id,
        &content,
        &claims.sub
    )
    .execute(&mut *transaction)
    .await?;
    if update_result.rows_affected() == 0 {
        return Err(CommentsError::UpdateUnauthorized(id))?;
    }
    transaction.commit().await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(serde::Deserialize, Validate)]
pub struct DeletePayload {
    #[validate(range(min = 0))]
    id: i64,
}

// TODO(ducanhgh): add a column named `isDeleted` and change this function so that
// it no longer truly deletes the comment. This helps preserve the replies.
#[instrument(name = "Deleting a comment", skip(pool, claims), fields(uid = %claims.sub))]
pub async fn delete(
    State(AppState { pool, .. }): State<AppState>,
    claims: UserClaims,
    Query(DeletePayload { id }): Query<DeletePayload>,
) -> Result<impl IntoResponse, AppError> {
    let mut transaction = pool.begin().await?;
    let delete_result = sqlx::query!(
        "DELETE FROM comments WHERE id = $1 AND author_id = $2",
        &id,
        &claims.sub
    )
    .execute(&mut *transaction)
    .await?;
    if delete_result.rows_affected() == 0 {
        return Err(CommentsError::UpdateUnauthorized(id))?;
    }
    transaction.commit().await?;
    Ok(StatusCode::NO_CONTENT)
}
