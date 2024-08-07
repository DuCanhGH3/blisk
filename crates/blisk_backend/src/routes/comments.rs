use super::{
    auth::{OptionalUserClaims, UserClaims},
    reactions::PostReaction,
};
use crate::{
    app::AppState,
    utils::{
        errors::{AppError, CommentsError},
        response::response,
        structs::AppJson,
    },
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::Postgres;
use tracing::instrument;

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Comment {
    pub id: i64,
    pub content: String,
    pub author_name: String,
    #[sqlx(default)]
    pub user_reaction: Option<PostReaction>,
    #[sqlx(default)]
    pub children: Option<sqlx::types::Json<Vec<Comment>>>,
}

pub async fn read_base<'c>(
    transaction: &mut sqlx::Transaction<'c, Postgres>,
    OptionalUserClaims(claims): &OptionalUserClaims,
    post_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Comment>, AppError> {
    let uid = claims.as_ref().map(|claims| claims.sub);

    let comments = sqlx::query_as!(
        Comment,
        r#"SELECT
            c.id,
            c.content,
            u.name AS "author_name: _",
            ucr.type AS "user_reaction: _",
            fetch_replies(
                request_uid => $4,
                request_pid => $1,
                parent_id => c.id,
                parent_path => c.path,
                current_level => 4
            ) AS "children: sqlx::types::Json<Vec<Comment>>"
        FROM comments c
        JOIN users u
        ON c.author_id = u.id
        LEFT JOIN comment_reactions ucr
        ON ucr.comment_id = c.id AND ucr.user_id = $4
        WHERE c.post_id = $1 AND c.path = 'Top'
        ORDER BY c.id DESC
        LIMIT $2
        OFFSET $3"#,
        &post_id,
        &limit,
        &offset,
        &uid as &_,
    )
    .fetch_all(&mut **transaction)
    .await?;

    Ok(comments)
}

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
    post_id: i64,
    offset: i64,
}

#[instrument(name = "Reading a comment", skip(pool, claims))]
pub async fn read(
    State(AppState { pool, .. }): State<AppState>,
    claims: OptionalUserClaims,
    Query(ReadQuery { post_id, offset }): Query<ReadQuery>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let comments_list = read_base(&mut transaction, &claims, post_id, 20, offset).await?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(comments_list)))
}

#[derive(serde::Deserialize)]
pub struct UpdatePayload {
    id: i64,
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

#[derive(serde::Deserialize)]
pub struct DeletePayload {
    id: i64,
}

#[instrument(name = "Deleting a comment", skip(pool, claims), fields(uid = %claims.sub))]
pub async fn delete(
    State(AppState { pool, .. }): State<AppState>,
    claims: UserClaims,
    AppJson(DeletePayload { id }): AppJson<DeletePayload>,
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
