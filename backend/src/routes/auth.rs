use crate::{
    app::AppState,
    settings::SETTINGS,
    utils::{
        self,
        constants::TEMPLATES,
        emails::send_email,
        errors::AppError,
        response::{created, response, SuccessResponse},
        structs::{AppForm, AppImage, AppJson, AppMultipart},
        uploads::upload_file,
    },
};
use axum::{
    async_trait,
    body::Bytes,
    extract::{FromRequestParts, Query, State},
    http::{header, request::Parts, HeaderValue, StatusCode},
    response::Response,
    RequestPartsExt,
};
use axum_extra::{
    extract::cookie::CookieJar,
    headers::{authorization::Bearer, Authorization},
    typed_header::TypedHeaderRejectionReason,
    TypedHeader,
};
use axum_typed_multipart::{FieldData, TryFromMultipart};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::{distributions::Alphanumeric, Rng};
use redis::Commands;
use tracing::instrument;
use validator::Validate;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("this user either doesn't exist or has already been verified")]
    AlreadyVerified,
    #[error("this user is not valid")]
    Invalid,
    #[error("this error is not expected")]
    Unexpected,
    #[error("an token was already used when it was requested again")]
    TokenUsed,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "urole", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
}

#[derive(Default, Debug, sqlx::FromRow, serde::Serialize)]
#[sqlx(default)]
pub struct User {
    pub id: Option<i64>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub role: Option<UserRole>,
    pub is_verified: Option<bool>,
    pub picture: Option<sqlx::types::Json<AppImage>>,
    pub password: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserClaims {
    pub iss: String,
    pub sub: i64,
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
}

impl UserClaims {
    pub fn new(iss: String, sub: i64, aud: String, exp: i64, iat: i64) -> UserClaims {
        UserClaims {
            iss,
            sub,
            aud,
            exp,
            iat,
            name: None,
            picture: None,
            email: None,
            email_verified: None,
        }
    }
    pub fn encode(&self) -> Result<String, AppError> {
        Ok(encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(SETTINGS.auth.access.sec.as_bytes()),
        )?)
    }
    pub fn decode(token: &str) -> Result<UserClaims, AppError> {
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_audience(&[SETTINGS.frontend.url.clone()]);
        let token_data = decode::<UserClaims>(
            token,
            &DecodingKey::from_secret(SETTINGS.auth.access.sec.as_bytes()),
            &validation,
        )?;
        Ok(token_data.claims)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UserClaims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let OptionalUserClaims(claims) =
            OptionalUserClaims::from_request_parts(parts, state).await?;
        Ok(claims.ok_or_else(|| AuthError::Invalid)?)
    }
}

/// Used for endpoints where an user's authentication
/// details can be used, but they are optional.
pub struct OptionalUserClaims(pub Option<UserClaims>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalUserClaims
where
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let header = match parts.extract::<TypedHeader<Authorization<Bearer>>>().await {
            Ok(header) => Some(header),
            Err(err) => match err.reason() {
                TypedHeaderRejectionReason::Missing => None,
                &_ => {
                    return Err(AuthError::Invalid)?;
                }
            },
        };

        if let Some(TypedHeader(Authorization(bearer))) = header {
            return Ok(OptionalUserClaims(Some(UserClaims::decode(
                bearer.token(),
            )?)));
        }

        let jar = parts.extract::<CookieJar>().await?;

        if let Some(token) = jar.get("token") {
            return Ok(OptionalUserClaims(Some(UserClaims::decode(token.value())?)));
        }

        Ok(OptionalUserClaims(None))
    }
}

pub static SESSION_REFRESH_TOKEN_PREFIX: &str = "session_refresh_token_uid";

const CONFIRMATION_TOKEN_PREFIX: &str = "confirmation_token_sid";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TokenClaims {
    pub exp: i64,
    pub uid: String,
    pub sid: String,
}

pub struct ConfirmationToken {
    pub uid: String,
}

#[instrument(name = "Issuing confirmation token", skip(redis_con))]
pub async fn issue_confirmation_token(
    redis_con: &mut redis::Connection,
    uid: String,
    is_password_change: bool,
) -> Result<String, AppError> {
    let sid: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    let redis_key = {
        if is_password_change {
            format!("{}_password_change_{}", CONFIRMATION_TOKEN_PREFIX, sid)
        } else {
            format!("{}_{}", CONFIRMATION_TOKEN_PREFIX, sid)
        }
    };
    let _: () = redis_con.set(&redis_key, String::new())?;
    let now = chrono::Local::now();
    let ttl = {
        if is_password_change {
            chrono::Duration::hours(1)
        } else {
            chrono::Duration::seconds(SETTINGS.secret.exp)
        }
    };
    let exp = (now + ttl).timestamp();
    let _: () = redis_con.expire(&redis_key, ttl.num_seconds())?;
    let claims = TokenClaims { exp, uid, sid };
    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SETTINGS.secret.sec.as_bytes()),
    )?)
}

#[instrument(name = "Verifying confirmation token", skip(redis_con))]
pub async fn verify_confirmation_token(
    redis_con: &mut redis::Connection,
    token: String,
    is_password_change: bool,
) -> Result<ConfirmationToken, AppError> {
    let token = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(SETTINGS.secret.sec.as_bytes()),
        &Validation::default(),
    )?;

    let redis_key = {
        if is_password_change {
            format!(
                "{}_password_change_{}",
                CONFIRMATION_TOKEN_PREFIX, token.claims.sid
            )
        } else {
            format!("{}_{}", CONFIRMATION_TOKEN_PREFIX, token.claims.sid)
        }
    };

    let redis_entry: Option<String> = redis_con.get(redis_key.clone())?;

    if redis_entry.is_none() {
        return Err(AuthError::TokenUsed)?;
    }

    let _: () = redis_con.del(redis_key.clone())?;

    Ok(ConfirmationToken {
        uid: token.claims.uid,
    })
}

#[instrument(
    name = "Sending a confirmation email",
    skip(redis_con),
    fields(
        recipient_user_id = %uid,
        recipient_name = %recipient_name,
        recipient_email = %recipient_email,
    )
)]
pub async fn send_confirmation_email(
    redis_con: &mut redis::Connection,
    subject: String,
    uid: String,
    recipient_name: String,
    recipient_email: String,
    is_password_change: bool,
) -> Result<(), AppError> {
    let title = subject.clone();

    let issued_token = match issue_confirmation_token(redis_con, uid, is_password_change).await {
        Ok(t) => t,
        Err(e) => {
            return Err(e);
        }
    };

    let confirmation_link = {
        if is_password_change {
            format!(
                "{}/auth/change-password?token={}",
                SETTINGS.frontend.url, issued_token,
            )
        } else {
            format!(
                "{}/auth/confirm?token={}",
                SETTINGS.frontend.url, issued_token,
            )
        }
    };

    let now = chrono::Local::now();
    let ttl = {
        if is_password_change {
            chrono::Duration::hours(1)
        } else {
            chrono::Duration::seconds(SETTINGS.secret.exp)
        }
    };
    let exp = now + ttl;

    let template = TEMPLATES.get_template("confirmation_email.html")?;

    let ctx = minijinja::context! {
        title => &title,
        confirmation_link => &confirmation_link,
        domain => &SETTINGS.frontend.url,
        ttl_minutes => ttl.num_minutes(),
        // Sat, 01 Jun 2024 14:17:00 UTC+7
        expiration_time => &exp.format("%a, %b %d %Y %X UTC%z").to_string()
    };
    let html_text = template.render(ctx).unwrap();

    let text = format!(
        r#"
        Tap the link below to confirm your email address.
        {}
        "#,
        confirmation_link
    );

    send_email(
        None,
        recipient_name,
        recipient_email,
        subject,
        html_text,
        text,
    )
    .await
}

#[derive(serde::Serialize)]
pub struct AuthenticateResponse {
    email: String,
    name: String,
    role: UserRole,
    is_verified: bool,
    picture: Option<sqlx::types::Json<AppImage>>,
}

#[instrument(name = "Fetching user info...", skip(pool, claims), fields(
    uid = &claims.sub
))]
pub async fn authenticate(
    State(AppState { pool, .. }): State<AppState>,
    claims: UserClaims,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let user = sqlx::query_as!(
        AuthenticateResponse,
        r#"SELECT
            email AS "email!",
            name AS "name!",
            role AS "role!: _",
            is_verified AS "is_verified!",
            picture AS "picture?: _"
        FROM users_view u
        WHERE u.id = $1"#,
        &claims.sub
    )
    .fetch_one(&mut *transaction)
    .await?;
    Ok(response(StatusCode::OK, None, AppJson(user)))
}

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

#[derive(TryFromMultipart, Validate)]
pub struct RegisterPayload {
    #[validate(length(min = 1))]
    email: String,
    #[validate(length(min = 1))]
    password: String,
    #[validate(length(min = 1))]
    username: String,
    /// User's profile picture.
    picture: FieldData<Bytes>,
}

#[instrument(
    name = "Registering a new user",
    skip(pool, redis_client, email, password, username, picture)
)]
pub async fn register(
    State(AppState {
        pool,
        s3,
        redis_client,
        ..
    }): State<AppState>,
    AppMultipart(RegisterPayload {
        email,
        password,
        username,
        picture,
    }): AppMultipart<RegisterPayload>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let password = utils::password::hash(&password)?;
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
    let picture_id = upload_file(&mut transaction, &s3, uid, None, picture).await?;
    sqlx::query!(
        "UPDATE users SET picture_id = $1 WHERE id = $2",
        &picture_id,
        &uid
    )
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    let location = format!("{}/users/{}", SETTINGS.frontend.url, username);
    let mut redis_con = redis_client.get_connection()?;
    send_confirmation_email(
        &mut redis_con,
        "blisk - Confirmation email".to_owned(),
        uid.to_string(),
        username,
        email,
        false,
    )
    .await?;
    Ok(created(location))
}

#[derive(serde::Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(length(min = 1, message = "Username is not valid!"))]
    username: String,
    #[validate(length(min = 1, message = "Password is not valid!"))]
    password: String,
}
#[derive(serde::Serialize)]
pub struct LoginResponse {
    token: String,
    expires_in: i64,
}

#[instrument(name = "Authenticating user...", skip(pool, password))]
pub async fn login(
    State(AppState { pool, .. }): State<AppState>,
    AppForm(LoginPayload { username, password }): AppForm<LoginPayload>,
) -> Result<Response, AppError> {
    let mut transaction = pool.begin().await?;
    let user = sqlx::query!(
        "SELECT id, name, email, password, is_verified FROM users WHERE name = $1",
        &username
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AuthError::Invalid,
        _ => AuthError::Unexpected,
    })?;
    transaction.commit().await?;
    let user_id = user.id;
    let password_hash = user.password;
    if !utils::password::verify(password_hash, password)? {
        return Err(AppError::from(AuthError::Invalid));
    }
    let now = chrono::Local::now();
    let id_ttl = chrono::Duration::seconds(SETTINGS.auth.access.exp);
    let id_claims = UserClaims::new(
        SETTINGS.app.base.clone(),
        user_id,
        SETTINGS.frontend.url.clone(),
        (now + id_ttl).timestamp(),
        now.timestamp(),
    );
    Ok(response(
        StatusCode::OK,
        Some(vec![(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        )]),
        AppJson(LoginResponse {
            token: id_claims.encode()?,
            expires_in: id_ttl.num_seconds(),
        }),
    ))
}
