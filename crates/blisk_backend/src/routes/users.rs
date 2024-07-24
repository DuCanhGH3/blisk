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
    user_reaction: Option<PostReaction>, // comments: Vec<ReadResponseComment>,
}
#[derive(serde::Serialize, sqlx::FromRow)]
struct ReadResponse {
    name: String,
    posts: sqlx::types::Json<Vec<ReadResponsePost>>,
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
            COALESCE(JSONB_AGG(p), '[]'::JSONB) AS "posts!: sqlx::types::Json<Vec<ReadResponsePost>>"
        FROM users u
        LEFT JOIN LATERAL (
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
        ) p ON TRUE
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
