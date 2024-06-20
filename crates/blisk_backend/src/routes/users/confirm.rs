use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Response,
};
use tracing::instrument;

use crate::{
    app::AppState,
    utils::{
        users::{confirmation_token::verify_confirmation_token, errors::AuthError},
        errors::AppError,
        json::AppJson,
        response::{response, SuccessResponse},
    },
};

#[derive(serde::Deserialize)]
pub struct ConfirmQuery {
    token: String,
}

#[instrument(name = "Confirming user", skip(pool, redis_client))]
pub async fn confirm(
    State(AppState {
        pool, redis_client, ..
    }): State<AppState>,
    Query(ConfirmQuery { token }): Query<ConfirmQuery>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let mut redis_con = redis_client.get_connection()?;
    let token = verify_confirmation_token(&mut redis_con, token, false).await?;
    sqlx::query("UPDATE users SET is_verified = TRUE WHERE id = $1 AND is_verified = FALSE")
        .bind(&token.uid)
        .execute(&mut *transaction)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => AppError::from(AuthError::AlreadyVerified),
            _ => AppError::from(err),
        })?;
    transaction.commit().await?;
    Ok(response(
        StatusCode::OK,
        None,
        AppJson(SuccessResponse {
            message: "Verified successfully!".to_owned(),
        }),
    ))
}
