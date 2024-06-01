use crate::{
    app::ApplicationState,
    utils::{auth, emails::send_confirmation_email},
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use sqlx::Row;
use tracing::{event, instrument, Level};

#[derive(serde::Deserialize)]
pub struct UserRegisterPayload {
    email: String,
    password: String,
    name: String,
}

#[instrument(name = "Registering a new user", skip(pool, redis_client))]
pub async fn register(
    State(ApplicationState {
        pool, redis_client, ..
    }): State<ApplicationState>,
    Json(UserRegisterPayload {
        email,
        password,
        name,
    }): Json<UserRegisterPayload>,
) -> impl IntoResponse {
    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            event!(
                Level::ERROR,
                "(sqlx) failed to begin a new transaction: {}",
                err
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Internal Server Error"
                })),
            );
        }
    };
    let password = match auth::password::hash(password) {
        Ok(password) => password,
        Err(err) => {
            event!(Level::ERROR, "(argon2) failed to hash password: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Internal Server Error"
                })),
            );
        }
    };
    let uid = match sqlx::query(
        "INSERT INTO users (email, name, password) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(&email)
    .bind(&name)
    .bind(&password)
    .try_map(|row: sqlx::postgres::PgRow| -> Result<String, sqlx::Error> { Ok(row.try_get("id")?) })
    .fetch_one(&mut *transaction)
    .await
    {
        Ok(uid) => uid,
        Err(err) => {
            event!(Level::ERROR, "(sqlx) failed to create a new user: {}", err);
            let error_message = if let Some(db_err) = err.as_database_error() {
                if db_err.is_unique_violation() {
                    "User already exists!"
                } else {
                    "Internal Server Error"
                }
            } else {
                "Internal Server Error"
            };
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": error_message
                })),
            );
        }
    };
    let mut redis_con = match redis_client.get_connection() {
        Ok(connection) => connection,
        Err(err) => {
            event!(
                Level::ERROR,
                "(redis) failed to retrieve Redis connection: {}",
                err
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Internal Server Error"
                })),
            );
        }
    };
    if let Err(err) = send_confirmation_email(
        &mut redis_con,
        "blisk - Confirmation email".to_owned(),
        uid,
        name,
        email,
        false,
    )
    .await
    {
        event!(Level::ERROR, "failed to send confirmation email: {}", err);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Internal Server Error"
            })),
        );
    }
    if let Err(err) = transaction.commit().await {
        event!(Level::ERROR, "(sqlx) failed to commit transaction: {}", err);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Internal Server Error"
            })),
        );
    }
    (
        StatusCode::CREATED,
        Json(json!({
            "message": "Account created successfully."
        })),
    )
}
