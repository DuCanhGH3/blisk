use crate::app::ApplicationState;
use crate::utils::auth;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(serde::Deserialize)]
pub struct UserRegisterPayload {
    email: String,
    password: String,
    name: String,
}

pub async fn register(
    State(ApplicationState { pool, .. }): State<ApplicationState>,
    Json(UserRegisterPayload {
        email,
        password,
        name,
    }): Json<UserRegisterPayload>,
) -> impl IntoResponse {
    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(_) => {
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
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Internal Server Error"
                })),
            )
        }
    };
    match sqlx::query("INSERT INTO users (email, name, password) VALUES ($1, $2, $3) RETURNING id")
        .bind(&email)
        .bind(&name)
        .bind(&password)
        .fetch_one(&mut *transaction)
        .await
    {
        Ok(_) => {}
        Err(err) => {
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
    if transaction.commit().await.is_err() {
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
