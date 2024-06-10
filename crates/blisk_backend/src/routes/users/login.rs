use axum::{
    extract::{Query, State},
    http::{header, HeaderValue, StatusCode},
    response::Response,
    Json,
};
use tracing::instrument;

use crate::{
    app::ApplicationState,
    settings::SETTINGS,
    utils::{
        auth::{
            errors::AuthError,
            password::verify,
            structs::{User, UserClaims},
        },
        errors::ApplicationError,
        response::response,
    },
};

#[derive(serde::Deserialize)]
pub struct LoginQuery {
    client_id: String,
}
#[derive(serde::Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}
#[derive(serde::Serialize)]
pub struct LoginResponse {
    token_type: String,
    expires_in: i64,
    id_token: String,
}

#[instrument(name = "Logging user in", skip(pool, client_id, password))]
pub async fn login(
    State(ApplicationState { pool, .. }): State<ApplicationState>,
    Query(LoginQuery { client_id }): Query<LoginQuery>,
    Json(LoginPayload { username, password }): Json<LoginPayload>,
) -> Result<Response, ApplicationError> {
    let mut transaction = pool.begin().await?;
    let user: User = sqlx::query_as("SELECT id, password FROM users WHERE name = $1")
        .bind(&username)
        .fetch_one(&mut *transaction)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AuthError::Invalid,
            _ => AuthError::Unexpected,
        })?;
    transaction.commit().await?;
    let user_id = user
        .id
        .ok_or(ApplicationError::Unexpected("user.id is unexpectedly None"))?;
    let password_hash = user.password.ok_or(ApplicationError::Unexpected(
        "user.password is unexpectedly None",
    ))?;
    if !verify(password_hash, password)? {
        return Err(ApplicationError::from(AuthError::Invalid));
    }
    let now = chrono::Local::now();
    let id_ttl = chrono::Duration::seconds(SETTINGS.auth.access.exp);
    let id_claims = UserClaims {
        iss: SETTINGS.application.base.clone(),
        sub: user_id,
        aud: client_id,
        exp: (now + id_ttl).timestamp(),
        iat: now.timestamp(),
    };
    let id_token = id_claims.encode()?;
    Ok(response(
        StatusCode::OK,
        Some(vec![(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        )]),
        Json(LoginResponse {
            token_type: "Bearer".to_owned(),
            expires_in: id_ttl.num_seconds(),
            id_token,
        }),
    ))
}
