use super::{
    auth::{OptionalUserClaims, UserClaims},
    books::BooksError,
    comments::Comment,
    reactions::{PostReaction, PostReactionMetadata},
};
use crate::{
    app::AppState,
    utils::{
        errors::AppError,
        response::response,
        structs::{AppImage, AppJson, AppQuery},
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::instrument;
use validator::Validate;

#[derive(Debug, thiserror::Error)]
pub enum PostsError {
    #[error("user has not finished reading book {0} before posting")]
    BookNotCompleted(String),
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
    pub book_title: Option<String>,
    pub book_name: Option<String>,
    pub book_synopsis: Option<String>,
    pub book_cover: Option<sqlx::types::Json<AppImage>>,
    pub book_spine: Option<sqlx::types::Json<AppImage>>,
    pub book_reaction: Reaction,
    pub reactions: Option<sqlx::types::Json<PostReactionMetadata>>,
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
    let user_record = sqlx::query!(
        "WITH book AS (SELECT b.id, b.title FROM books b WHERE b.name = $1)
        SELECT b.title AS book_title, ub.completed
        FROM users_books ub, book b
        WHERE ub.book_id = b.id AND ub.user_id = $2",
        &book,
        &claims.sub
    )
    .fetch_one(&mut *transaction)
    .await?;
    if !user_record.completed {
        return Err(PostsError::BookNotCompleted(user_record.book_title))?;
    }
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

#[derive(serde::Deserialize, Validate)]
pub struct ReadQuery {
    #[validate(length(min = 0, message = "`user` must point to a valid user!"))]
    user: Option<String>,
    #[validate(range(min = 0, message = "`comment_id` must point to a valid comment!"))]
    comment_id: Option<i64>,
    #[validate(range(min = 0, message = "`previous_last` must point to a valid post!"))]
    previous_last: Option<i64>,
}

#[instrument(name = "Reading a post", skip(pool, claims))]
pub async fn read(
    State(AppState { pool, .. }): State<AppState>,
    claims: OptionalUserClaims,
    AppQuery(ReadQuery {
        user,
        comment_id,
        previous_last,
    }): AppQuery<ReadQuery>,
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
            b.title AS "book_title?",
            b.name AS "book_name?",
            b.summary AS "book_synopsis?",
            b.cover_image AS "book_cover?: _",
            b.spine_image AS "book_spine?: _",
            p.book_reaction AS "book_reaction!: _",
            p.reactions AS "reactions?: _",
            p.user_reaction AS "user_reaction!: _",
            coalesce(jsonb_agg(c) FILTER (WHERE c.id IS NOT NULL), '[]'::JSONB) AS "comments!: _"
        FROM fetch_posts(request_uid => $1) p
        JOIN books_view b
        ON b.id = p.book_id
        LEFT JOIN LATERAL (
            SELECT * FROM fetch_comments(request_uid => $1, replies_depth => 4) c
            WHERE CASE
                WHEN $3::BIGINT IS NULL AND c.post_id = p.id AND c.path = 'Top' THEN TRUE
                WHEN $3::BIGINT IS NOT NULL AND c.post_id = p.id AND c.id = $3::BIGINT THEN TRUE
                ELSE FALSE
            END
            ORDER BY c.id DESC
            LIMIT 20
        ) c ON TRUE
        WHERE CASE
            WHEN $2::TEXT IS NULL THEN TRUE
            WHEN $2::TEXT IS NOT NULL AND p.author_name = $2::TEXT THEN TRUE
            ELSE FALSE
        END AND CASE
            WHEN $4::BIGINT IS NULL THEN TRUE
            WHEN $4::BIGINT IS NOT NULL AND p.id < $4::BIGINT THEN TRUE
            ELSE FALSE
        END
        GROUP BY p.id, p.title, p.content, p.author_name, p.author_picture, b.title, b.name,
        b.summary, b.cover_image, b.spine_image, p.book_reaction, p.reactions, p.user_reaction
        ORDER BY p.id DESC
        LIMIT 20"#,
        &uid as &_,
        &user as &_,
        &comment_id as &_,
        &previous_last as &_,
    )
    .fetch_all(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(post)))
}

#[derive(serde::Deserialize, Validate)]
pub struct ReadSlugQuery {
    #[validate(range(min = 0, message = "`comment_id` must point to a valid comment!"))]
    comment_id: Option<i64>,
}

#[instrument(name = "Reading a post", skip(pool, claims))]
pub async fn read_slug(
    State(AppState { pool, .. }): State<AppState>,
    claims: OptionalUserClaims,
    Path(post_id): Path<i64>,
    AppQuery(ReadSlugQuery { comment_id }): AppQuery<ReadSlugQuery>,
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
            b.title AS "book_title?",
            b.name AS "book_name?",
            b.summary AS "book_synopsis?",
            b.cover_image AS "book_cover?: _",
            b.spine_image AS "book_spine?: _",
            p.book_reaction AS "book_reaction!: _",
            p.reactions AS "reactions?: _",
            p.user_reaction AS "user_reaction!: _",
            coalesce(jsonb_agg(c) FILTER (WHERE c.id IS NOT NULL), '[]'::JSONB) AS "comments!: _"
        FROM fetch_posts(request_uid => $2) p
        JOIN books_view b
        ON b.id = p.book_id
        LEFT JOIN LATERAL (
            SELECT * FROM fetch_comments(request_uid => $2, replies_depth => 4) c
            WHERE CASE
                WHEN $3::BIGINT IS NULL AND c.post_id = p.id AND c.path = 'Top' THEN TRUE
                WHEN $3::BIGINT IS NOT NULL AND c.post_id = p.id AND c.id = $3::BIGINT THEN TRUE
                ELSE FALSE
            END
            ORDER BY c.id DESC
            LIMIT 20
        ) c ON TRUE
        WHERE p.id = $1
        GROUP BY p.id, p.title, p.content, p.author_name, p.author_picture, b.title, b.name,
        b.summary, b.cover_image, b.spine_image, p.book_reaction, p.reactions, p.user_reaction"#,
        &post_id,
        &uid as &_,
        &comment_id as &_,
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::from(PostsError::PostNotFound(post_id)),
        _ => AppError::from(e),
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
        sqlx::Error::RowNotFound => AppError::from(PostsError::UpdateUnauthorized(id)),
        _ => AppError::from(err),
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
