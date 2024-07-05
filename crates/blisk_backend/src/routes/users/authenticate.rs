use axum::{extract::State, http::StatusCode, response::Response};
use tracing::instrument;

use crate::{
    app::AppState,
    utils::{
        errors::AppError,
        response::response,
        structs::AppJson,
        users::structs::{User, UserClaims},
    },
};

#[instrument(name = "Fetching user info...", skip(pool, claims), fields(
    uid = &claims.sub
))]
pub async fn authenticate(
    State(AppState { pool, .. }): State<AppState>,
    claims: UserClaims,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let user: User = sqlx::query_as("SELECT email, name, role FROM users WHERE id = $1")
        .bind(&claims.sub)
        .fetch_one(&mut *transaction)
        .await?;
    Ok(response(StatusCode::OK, None, AppJson(user)))
}
