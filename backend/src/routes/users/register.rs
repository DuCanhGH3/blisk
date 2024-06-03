use crate::{
    app::ApplicationState,
    utils::{
        auth,
        errors::ApplicationError,
        response::{response, ErrorResponse, SuccessResponse},
    },
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sqlx::Row;
use tracing::instrument;

#[derive(serde::Deserialize)]
pub struct RegisterPayload {
    email: String,
    password: String,
    name: String,
}

#[instrument(name = "Registering a new user", skip(pool, redis_client))]
pub async fn register(
    State(ApplicationState {
        pool, redis_client, ..
    }): State<ApplicationState>,
    Json(RegisterPayload {
        email,
        password,
        name,
    }): Json<RegisterPayload>,
) -> Response {
    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            return ApplicationError::from(err).into_response();
        }
    };
    let password = match auth::password::hash(password) {
        Ok(password) => password,
        Err(err) => {
            return ApplicationError::from(err).into_response();
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
            if let Some(db_err) = err.as_database_error() {
                if db_err.is_unique_violation() {
                    return response(
                        StatusCode::BAD_REQUEST,
                        None,
                        Json(ErrorResponse {
                            error: "User already exists!".to_owned(),
                        }),
                    );
                }
            }
            return ApplicationError::from(err).into_response();
        }
    };
    let mut redis_con = match redis_client.get_connection() {
        Ok(connection) => connection,
        Err(err) => {
            return ApplicationError::from(err).into_response();
        }
    };
    if let Err(err) = auth::emails::send_confirmation_email(
        &mut redis_con,
        "blisk - Confirmation email".to_owned(),
        uid,
        name,
        email,
        false,
    )
    .await
    {
        return err.into_response();
    }
    if let Err(err) = transaction.commit().await {
        return ApplicationError::from(err).into_response();
    }
    response(
        StatusCode::CREATED,
        None,
        Json(SuccessResponse {
            message: "Account created successfully.".to_owned(),
        }),
    )
}
