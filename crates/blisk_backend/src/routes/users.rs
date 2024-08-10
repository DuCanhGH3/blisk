use crate::utils::errors::AppError;
use crate::{
    app::AppState,
    utils::{errors::UserError, response::response, structs::AppJson},
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Response,
};

use super::auth::OptionalUserClaims;
use super::comments::Comment;
use super::posts::Reaction;
use super::reactions::PostReaction;

#[derive(serde::Deserialize)]
pub struct ReadQuery {
    username: String,
}
// #[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
// struct ReadResponseComment {
//     content: String,
// }
#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
struct ReadResponsePost {
    id: i64,
    title: String,
    content: String,
    reaction: Reaction,
    user_reaction: Option<PostReaction>,
}
#[derive(serde::Serialize, sqlx::FromRow)]
struct ReadResponse {
    name: String,
    posts: sqlx::types::Json<Vec<ReadResponsePost>>,
    comments: sqlx::types::Json<Vec<Comment>>,
}

pub async fn read(
    State(AppState { pool, .. }): State<AppState>,
    OptionalUserClaims(claims): OptionalUserClaims,
    Query(ReadQuery { username }): Query<ReadQuery>,
) -> Result<Response, AppError> {
    let uid = claims.as_ref().map(|claims| claims.sub);
    let mut transaction = pool.begin().await?;
    let user = sqlx::query_as!(
        ReadResponse,
        r#"SELECT
            u.name,
            COALESCE((
                SELECT JSONB_AGG(p) FILTER (WHERE p.id IS NOT NULL)
                FROM (
                    SELECT
                        id,
                        title,
                        content,
                        reaction,
                        user_reaction
                    FROM fetch_posts(request_uid => $2)
                    ORDER BY id DESC
                    LIMIT 5
                    OFFSET 0
                ) p
            ), '[]'::JSONB) AS "posts!: sqlx::types::Json<Vec<ReadResponsePost>>",
            COALESCE((
                SELECT JSONB_AGG(c) FILTER (WHERE c.id IS NOT NULL)
                FROM (
                    SELECT
                        c.id,
                        c.content,
                        c.author_name,
                        c.user_reaction,
                        c.children
                    FROM fetch_comments(
                        request_uid => $2,
                        replies_depth => 0
                    ) c
                    WHERE c.author_id = u.id
                    ORDER BY c.id DESC
                    LIMIT 20
                    OFFSET 0
                ) c
            ), '[]'::JSONB) AS "comments!: sqlx::types::Json<Vec<Comment>>"
        FROM users u
        WHERE u.name = $1
        GROUP BY u.id"#,
        &username,
        &uid as &_,
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => UserError::UserNotFound(username),
        _ => UserError::Unexpected,
    })?;
    transaction.commit().await?;
    Ok(response(StatusCode::OK, None, AppJson(user)))
}
