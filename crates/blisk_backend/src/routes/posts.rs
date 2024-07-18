use super::{
    auth::{OptionalUserClaims, UserClaims},
    comments::{self, Comment},
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
    response::Response,
};
use tracing::instrument;

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, serde::Deserialize, serde::Serialize)]
#[sqlx(type_name = "breact", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Reaction {
    Like,
    Dislike,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_name: String,
    pub reaction: Reaction,
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
    let mut transaction = pool.begin().await?;
    let post = sqlx::query_as!(
        Post,
        r#"SELECT
            p.id,
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
    let comments = comments::read_base(&mut transaction, &claims, post_id, 20, 0).await?;
    transaction.commit().await?;
    Ok(response(
        StatusCode::OK,
        None,
        AppJson(ReadResponse { post, comments }),
    ))
}
