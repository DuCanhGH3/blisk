use super::{
    auth::{OptionalUserClaims, UserClaims},
    comments::{self, Comment},
    reactions::PostReaction,
};
use crate::{
    app::AppState,
    utils::{
        errors::{AppError, PostsError},
        response::response,
        structs::AppJson,
    },
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::instrument;

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, serde::Deserialize, serde::Serialize)]
#[sqlx(type_name = "breact", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Reaction {
    Like,
    Dislike,
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_name: String,
    pub reaction: Reaction,
    pub user_reaction: Option<PostReaction>,
}

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

#[derive(serde::Deserialize)]
pub struct ReadQuery {
    post_id: i64,
}
#[derive(serde::Serialize)]
pub struct ReadResponse {
    post: Post,
    comments: Vec<Comment>,
}

#[instrument(name = "Reading a post", skip(pool, claims))]
pub async fn read(
    State(AppState { pool, .. }): State<AppState>,
    claims: OptionalUserClaims,
    Query(ReadQuery { post_id }): Query<ReadQuery>,
) -> Result<Response, AppError> {
    let uid = claims.0.as_ref().map(|claims| claims.sub);
    let mut transaction = pool.begin().await?;
    let post = sqlx::query_as!(
        Post,
        r#"SELECT
            p.id,
            p.title,
            p.content,
            u.name as author_name,
            p.reaction AS "reaction: _",
            ucr.type AS "user_reaction: _"
        FROM posts p
        JOIN users u ON p.author_id = u.id
        LEFT JOIN post_reactions ucr
        ON ucr.post_id = $1 AND ucr.user_id = $2
        WHERE p.id = $1"#,
        &post_id,
        &uid as &_,
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => PostsError::PostNotFound(post_id),
        _ => PostsError::Unexpected,
    })?;
    let comments = comments::read_base(&mut transaction, &claims, post_id, 20, 0).await?;
    transaction.commit().await?;
    Ok(response(
        StatusCode::OK,
        None,
        AppJson(ReadResponse { post, comments }),
    ))
}

#[derive(serde::Deserialize)]
pub struct UpdatePayload {
    id: i64,
    title: Option<String>,
    content: Option<String>,
    reaction: Option<Reaction>,
}

#[instrument(name = "Updating a post", skip(pool, claims), fields(uid = %claims.sub))]
pub async fn update(
    State(AppState { pool, .. }): State<AppState>,
    claims: UserClaims,
    AppJson(UpdatePayload {
        id,
        title,
        content,
        reaction,
    }): AppJson<UpdatePayload>,
) -> Result<impl IntoResponse, AppError> {
    let mut transaction = pool.begin().await?;
    let post = sqlx::query!(
        r#"SELECT title, content, reaction AS "reaction: Reaction" FROM posts WHERE id = $1 AND author_id = $2"#,
        &id,
        &claims.sub
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => PostsError::UpdateUnauthorized(id),
        _ => PostsError::Unexpected,
    })?;
    let update_result = sqlx::query!(
        "UPDATE posts SET title = $3, content = $4, reaction = $5 WHERE id = $1 AND author_id = $2",
        &id,
        &claims.sub,
        &title.unwrap_or_else(|| post.title),
        &content.unwrap_or_else(|| post.content),
        &reaction.unwrap_or_else(|| post.reaction) as &_,
    )
    .execute(&mut *transaction)
    .await?;
    if update_result.rows_affected() == 0 {
        return Err(PostsError::UpdateUnauthorized(id))?;
    }
    transaction.commit().await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(serde::Deserialize)]
pub struct DeletePayload {
    id: i64,
}

#[instrument(name = "Deleting a post", skip(pool, claims), fields(uid = %claims.sub))]
pub async fn delete(
    State(AppState { pool, .. }): State<AppState>,
    claims: UserClaims,
    AppJson(DeletePayload { id }): AppJson<DeletePayload>,
) -> Result<impl IntoResponse, AppError> {
    let mut transaction = pool.begin().await?;
    let delete_result = sqlx::query!(
        "DELETE FROM posts WHERE id = $1 AND author_id = $2",
        &id,
        &claims.sub
    )
    .execute(&mut *transaction)
    .await?;
    if delete_result.rows_affected() == 0 {
        return Err(PostsError::UpdateUnauthorized(id))?;
    }
    transaction.commit().await?;
    Ok(StatusCode::NO_CONTENT)
}
