use super::{
    auth::{OptionalUserClaims, UserClaims},
    books::BooksError,
    comments::Comment,
    reactions::PostReaction,
};
use crate::{
    app::AppState,
    utils::{
        errors::AppError,
        response::response,
        structs::{AppImage, AppJson},
    },
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::instrument;
use validator::Validate;

#[derive(Debug, thiserror::Error)]
pub enum PostsError {
    #[error("post {0} cannot be found")]
    PostNotFound(i64),
    #[error("post {0} was non-existent, or an unauthorized personnel tried to update it")]
    UpdateUnauthorized(i64),
    #[error("this error is not expected")]
    Unexpected,
}

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
    pub author_picture: Option<sqlx::types::Json<AppImage>>,
    pub reaction: Reaction,
    pub user_reaction: Option<PostReaction>,
    pub comments: Option<sqlx::types::Json<Vec<Comment>>>,
}

#[derive(serde::Deserialize, Validate)]
pub struct CreatePayload {
    #[validate(length(min = 1, message = "`book` must point to a valid slug!"))]
    book: String,
    #[validate(length(min = 1, max = 500, message = "Title is either too short or too long!"))]
    title: String,
    #[validate(length(min = 1, message = "Content is not valid!"))]
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
        book,
        title,
        content,
        reaction,
    }): AppJson<CreatePayload>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let pid: i64 = sqlx::query_scalar!(
        "INSERT INTO posts (author_id, book_id, title, content, reaction)
        VALUES ($1, (SELECT id FROM books WHERE name = $2), $3, $4, $5)
        RETURNING id",
        &claims.sub,
        &book,
        &title,
        &content,
        &reaction as _
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref err) => {
            if matches!(err.kind(), sqlx::error::ErrorKind::NotNullViolation) {
                AppError::from(BooksError::BookNotFound(book))
            } else {
                AppError::from(e)
            }
        }
        err => AppError::from(err),
    })?;
    transaction.commit().await?;
    Ok(response(
        StatusCode::CREATED,
        None,
        AppJson(CreateResponse { id: pid }),
    ))
}

#[derive(serde::Deserialize)]
pub struct ReadQuery {
    comment_id: Option<i64>,
    previous_last: Option<i64>,
}

#[instrument(name = "Reading a post", skip(pool, claims))]
pub async fn read(
    State(AppState { pool, .. }): State<AppState>,
    claims: OptionalUserClaims,
    Query(ReadQuery {
        comment_id,
        previous_last,
    }): Query<ReadQuery>,
) -> Result<Response, AppError> {
    let uid = claims.0.as_ref().map(|claims| claims.sub);
    let mut transaction = pool.begin().await?;
    let post = sqlx::query_as!(
        Post,
        r#"SELECT
            p.id AS "id!",
            p.title AS "title!",
            p.content AS "content!", 
            p.author_name AS "author_name!",
            p.author_picture AS "author_picture?: _",
            p.reaction AS "reaction!: _",
            p.user_reaction AS "user_reaction!: _",
            COALESCE(JSONB_AGG(c) FILTER (WHERE c.id IS NOT NULL), '[]'::JSONB) AS "comments!: _"
        FROM fetch_posts(request_uid => $1) p
        LEFT JOIN LATERAL (
            SELECT * FROM fetch_comments(
                request_uid => $1,
                replies_depth => 4
            ) c
            WHERE CASE
                WHEN $2::BIGINT IS NULL AND c.post_id = p.id AND c.path = 'Top' THEN TRUE
                WHEN $2::BIGINT IS NOT NULL AND c.post_id = p.id AND c.id = $2::BIGINT THEN TRUE
                ELSE FALSE
            END
            ORDER BY c.id DESC
            LIMIT 20
        ) c ON TRUE
        WHERE CASE
            WHEN $3::BIGINT IS NULL THEN TRUE
            WHEN $3::BIGINT IS NOT NULL AND p.id < $3::BIGINT THEN TRUE
            ELSE FALSE
        END
        GROUP BY p.id, p.title, p.content, p.author_name, p.author_picture, p.reaction, p.user_reaction
        ORDER BY p.id DESC
        LIMIT 20"#,
        &uid as &_,
        &comment_id as &_,
        &previous_last as &_,
    )
    .fetch_all(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(post)))
}

#[derive(serde::Deserialize)]
pub struct ReadSlugQuery {
    comment_id: Option<i64>,
}

#[instrument(name = "Reading a post", skip(pool, claims))]
pub async fn read_slug(
    State(AppState { pool, .. }): State<AppState>,
    claims: OptionalUserClaims,
    Path(post_id): Path<i64>,
    Query(ReadSlugQuery { comment_id }): Query<ReadSlugQuery>,
) -> Result<Response, AppError> {
    let uid = claims.0.as_ref().map(|claims| claims.sub);
    let mut transaction = pool.begin().await?;
    let post = sqlx::query_as!(
        Post,
        r#"SELECT
            p.id AS "id!",
            p.title AS "title!",
            p.content AS "content!", 
            p.author_name AS "author_name!",
            p.author_picture AS "author_picture?: _",
            p.reaction AS "reaction!: _",
            p.user_reaction AS "user_reaction!: _",
            COALESCE(JSONB_AGG(c) FILTER (WHERE c.id IS NOT NULL), '[]'::JSONB) AS "comments!: _"
        FROM fetch_posts(request_uid => $2) p
        LEFT JOIN LATERAL (
            SELECT * FROM fetch_comments(
                request_uid => $2,
                replies_depth => 4
            ) c
            WHERE
            CASE
                WHEN $3::BIGINT IS NULL AND c.post_id = p.id AND c.path = 'Top' THEN TRUE
                WHEN $3::BIGINT IS NOT NULL AND c.post_id = p.id AND c.id = $3::BIGINT THEN TRUE
                ELSE FALSE
            END
            ORDER BY c.id DESC
            LIMIT 20
        ) c ON TRUE
        WHERE p.id = $1
        GROUP BY p.id, p.title, p.content, p.author_name, p.author_picture, p.reaction, p.user_reaction"#,
        &post_id,
        &uid as &_,
        &comment_id as &_,
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

#[instrument(name = "Updating a post", skip(pool, claims, title, content, reaction), fields(uid = %claims.sub))]
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
