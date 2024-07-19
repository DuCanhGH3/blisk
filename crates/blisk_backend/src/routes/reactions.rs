use super::auth::UserClaims;
use crate::{
    app::AppState,
    utils::{errors::AppError, response::response, structs::AppJson},
};
use axum::{extract::State, http::StatusCode, response::Response};
use tracing::instrument;

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

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateForType {
    Post,
    Comment,
}

#[derive(serde::Deserialize)]
pub struct CreatePayload {
    for_type: CreateForType,
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
        CreateForType::Comment => sqlx::query!(
            "SELECT create_comment_reaction (rtype => $1, usid => $2, cid => $3)",
            &reaction_type as _,
            &claims.sub,
            &post_id
        ),
        CreateForType::Post => sqlx::query!(
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
