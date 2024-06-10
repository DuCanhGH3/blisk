use crate::{settings::SETTINGS, utils::errors::AppError};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::{distributions::Alphanumeric, Rng};
use redis::Commands;
use tracing::instrument;

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
    redis_con.set(&redis_key, String::new())?;
    let now = chrono::Local::now();
    let ttl = {
        if is_password_change {
            chrono::Duration::hours(1)
        } else {
            chrono::Duration::seconds(SETTINGS.secret.exp)
        }
    };
    let exp = (now + ttl).timestamp();
    redis_con.expire(&redis_key, ttl.num_seconds())?;
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
        return Err(AppError::TokenUsed);
    }

    redis_con.del(redis_key.clone())?;

    Ok(ConfirmationToken {
        uid: token.claims.uid,
    })
}
