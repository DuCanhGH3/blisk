use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use tracing::instrument;

use crate::{
    app::ApplicationState,
    utils::{
        auth::confirmation_token::verify_confirmation_token,
        errors::ApplicationError,
        response::{response, SuccessResponse},
    },
};

#[derive(serde::Deserialize)]
pub struct ConfirmQuery {
    token: String,
}

#[instrument(name = "Confirming user", skip(pool, redis_client))]
pub async fn confirm(
    State(ApplicationState {
        pool, redis_client, ..
    }): State<ApplicationState>,
    Query(ConfirmQuery { token }): Query<ConfirmQuery>,
) -> Response {
    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            return ApplicationError::SqlxError(err).into_response();
        }
    };
    let mut redis_con = match redis_client.get_connection() {
        Ok(con) => con,
        Err(err) => {
            return ApplicationError::RedisError(err).into_response();
        }
    };
    let token = match verify_confirmation_token(&mut redis_con, token, false).await {
        Ok(token) => token,
        Err(err) => {
            return err.into_response();
        }
    };
    if let Err(err) = sqlx::query("UPDATE users SET is_verified = TRUE WHERE id = $1")
        .bind(&token.uid)
        .execute(&mut *transaction)
        .await
    {
        return ApplicationError::SqlxError(err).into_response();
    };
    if let Err(err) = transaction.commit().await {
        return ApplicationError::SqlxError(err).into_response();
    }
    response(
        StatusCode::OK,
        None,
        Json(SuccessResponse {
            message: "Verified successfully".to_owned(),
        }),
    )
}
