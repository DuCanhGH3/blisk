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
) -> Response {
    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(err) => {
            event!(
                Level::ERROR,
                "(sqlx) failed to begin a new transaction: {}",
                err
            );
            return response(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Json(ErrorResponse {
                    error: "Internal Server Error".to_owned(),
                }),
            );
        }
    };
    let password = match auth::password::hash(password) {
        Ok(password) => password,
        Err(err) => {
            event!(Level::ERROR, "(argon2) failed to hash password: {}", err);
            return response(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Json(ErrorResponse {
                    error: "Internal Server Error".to_owned(),
                }),
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
            return ApplicationError::SqlxError(err).into_response();
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
            return response(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Json(ErrorResponse {
                    error: "Internal Server Error".to_owned(),
                }),
            );
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
        event!(Level::ERROR, "(sqlx) failed to commit transaction: {}", err);
        return response(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Json(ErrorResponse {
                error: "Internal Server Error".to_owned(),
            }),
        );
    }
    response(
        StatusCode::CREATED,
        None,
        Json(SuccessResponse {
            message: "Account created successfully.".to_owned(),
        }),
    )
}
