use super::auth::UserClaims;
use crate::{
    app::AppState,
    utils::{errors::AppError, response::response, structs::AppJson},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::instrument;
use validator::Validate;

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, serde::Deserialize, serde::Serialize)]
#[sqlx(type_name = "preact", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PostReaction {
    Like,
    Love,
    Laugh,
    Wow,
    Sad,
    Angry,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PostReactionMetadata {
    total: i64,
    like: i64,
    love: i64,
    laugh: i64,
    wow: i64,
    sad: i64,
    angry: i64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostReactionFor {
    Post,
    Comment,
}

#[derive(serde::Deserialize, Validate)]
pub struct CreatePayload {
    for_type: PostReactionFor,
    #[validate(range(min = 0))]
    post_id: i64,
    reaction_type: PostReaction,
}

#[derive(serde::Serialize)]
pub struct CreateResponse {
    reaction_type: PostReaction,
}

#[instrument(name = "Creating a reaction...", skip(pool, claims), fields(
    uid = &claims.sub
))]
pub async fn create(
    State(AppState { pool, .. }): State<AppState>,
    claims: UserClaims,
    AppJson(CreatePayload {
        for_type,
        post_id,
        reaction_type,
    }): AppJson<CreatePayload>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let insert_query = match for_type {
        PostReactionFor::Comment => sqlx::query!(
            "SELECT create_comment_reaction (rtype => $1, usid => $2, cid => $3)",
            &reaction_type as _,
            &claims.sub,
            &post_id
        ),
        PostReactionFor::Post => sqlx::query!(
            "SELECT create_post_reaction (rtype => $1, usid => $2, pid => $3)",
            &reaction_type as _,
            &claims.sub,
            &post_id
        ),
    };
    insert_query.execute(&mut *transaction).await?;
    transaction.commit().await?;
    Ok(response(
        StatusCode::CREATED,
        None,
        AppJson(CreateResponse { reaction_type }),
    ))
}

#[derive(serde::Deserialize, Validate)]
pub struct DeletePayload {
    for_type: PostReactionFor,
    #[validate(range(min = 0))]
    post_id: i64,
}

#[instrument(name = "Removing a reaction", skip(pool, claims), fields(uid = %claims.sub))]
pub async fn delete(
    State(AppState { pool, .. }): State<AppState>,
    claims: UserClaims,
    AppJson(DeletePayload { for_type, post_id }): AppJson<DeletePayload>,
) -> Result<impl IntoResponse, AppError> {
    let mut transaction = pool.begin().await?;
    let delete_query = match for_type {
        PostReactionFor::Comment => sqlx::query!(
            "DELETE FROM comment_reactions WHERE user_id = $1 AND comment_id = $2",
            &claims.sub,
            &post_id
        ),
        PostReactionFor::Post => sqlx::query!(
            "DELETE FROM post_reactions WHERE user_id = $1 AND post_id = $2",
            &claims.sub,
            &post_id
        ),
    };
    delete_query.execute(&mut *transaction).await?;
    transaction.commit().await?;
    Ok(StatusCode::NO_CONTENT)
}
