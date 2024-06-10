use crate::{
    app::AppState,
    utils::{
        auth::{self, errors::AuthError}, errors::AppError, json::AppJson, response::{response, SuccessResponse}
    },
};
use axum::{extract::State, http::StatusCode, response::Response};
use tracing::instrument;

#[derive(serde::Deserialize)]
pub struct RegisterPayload {
    email: String,
    password: String,
    username: String,
}

#[instrument(
    name = "Registering a new user",
    skip(pool, redis_client, email, password, username)
)]
pub async fn register(
    State(AppState {
        pool, redis_client, ..
    }): State<AppState>,
    AppJson(RegisterPayload {
        email,
        password,
        username,
    }): AppJson<RegisterPayload>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let password = auth::password::hash(&password)?;
    let uid: i64 = match sqlx::query_scalar(
        "INSERT INTO users (email, name, password) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(&email)
    .bind(&username)
    .bind(&password)
    .fetch_one(&mut *transaction)
    .await
    {
        Ok(uid) => uid,
        Err(err) => {
            println!("{}", err);
            if let Some(db_err) = err.as_database_error() {
                if db_err.is_unique_violation() {
                    return Err(AppError::from(AuthError::Invalid));
                }
            }
            return Err(AppError::from(err));
        }
    };
    transaction.commit().await?;
    let mut redis_con = redis_client.get_connection()?;
    auth::emails::send_confirmation_email(
        &mut redis_con,
        "blisk - Confirmation email".to_owned(),
        uid.to_string(),
        username,
        email,
        false,
    )
    .await?;
    Ok(response(
        StatusCode::CREATED,
        None,
        AppJson(SuccessResponse {
            message: "Account created successfully.".to_owned(),
        }),
    ))
}
