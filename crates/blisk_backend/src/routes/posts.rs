use super::{
    auth::{OptionalUserClaims, UserClaims},
    comments::Comment,
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
use validator::Validate;

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
    pub comments: Option<sqlx::types::Json<Vec<Comment>>>,
}

#[derive(serde::Deserialize, Validate)]
pub struct CreatePayload {
    #[validate(range(min = 0))]
    book_id: i64,
    #[validate(length(min = 1))]
    title: String,
    #[validate(length(min = 1))]
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
            p.id AS "id!: _",
            p.title AS "title!: _",
            p.content AS "content!: _", 
            p.author_name AS "author_name!: _",
            p.reaction AS "reaction!: _",
            p.user_reaction AS "user_reaction!: _",
            COALESCE(JSONB_AGG(c) FILTER (WHERE c.id IS NOT NULL), '[]'::JSONB) AS "comments!: sqlx::types::Json<Vec<Comment>>"
        FROM fetch_posts(request_uid => $2) p
        LEFT JOIN LATERAL (
            SELECT * FROM fetch_comments(
                request_uid => $2,
                replies_depth => 4
            ) c
            WHERE c.post_id = p.id
            ORDER BY c.id DESC
            LIMIT 20
            OFFSET 0
        ) c ON TRUE
        WHERE p.id = $1
        GROUP BY p.id, p.title, p.content, p.author_name, p.reaction, p.user_reaction"#,
        &post_id,
        &uid as &_,
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => PostsError::PostNotFound(post_id),
        _ => PostsError::Unexpected,
    })?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(post)))
}

#[derive(serde::Deserialize, Validate)]
pub struct UpdatePayload {
    #[validate(range(min = 0))]
    id: i64,
    #[validate(length(min = 1))]
    title: Option<String>,
    #[validate(length(min = 1))]
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

#[derive(serde::Deserialize, Validate)]
pub struct DeletePayload {
    #[validate(range(min = 0))]
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
