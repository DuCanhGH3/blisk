use axum::{
    extract::{Query, State},
    http::{header, HeaderValue, StatusCode},
    response::Response,
    Json,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use tracing::instrument;

use crate::{
    app::ApplicationState,
    settings::SETTINGS,
    utils::{
        auth::{errors::AuthError, password::verify, structs::User},
        errors::ApplicationError,
        response::response,
    },
};

#[derive(serde::Serialize)]
pub struct LoginClaims {
    pub iss: String,
    pub sub: i32,
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
}
#[derive(serde::Deserialize)]
pub struct LoginQuery {
    client_id: String,
}
#[derive(serde::Deserialize)]
pub struct LoginPayload {
    email: String,
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
    Json(LoginPayload { email, password }): Json<LoginPayload>,
) -> Result<Response, ApplicationError> {
    let mut transaction = pool.begin().await?;
    let user: User = sqlx::query_as("SELECT id, password FROM users WHERE email = $1")
        .bind(&email)
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
    let id_token = encode(
        &Header::default(),
        &LoginClaims {
            iss: SETTINGS.application.base.clone(),
            sub: user_id,
            aud: client_id,
            exp: (now + id_ttl).timestamp(),
            iat: now.timestamp(),
        },
        &EncodingKey::from_secret(SETTINGS.auth.access.sec.as_bytes()),
    )?;
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
