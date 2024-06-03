use crate::{settings::SETTINGS, utils::errors::ApplicationError};
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
) -> Result<String, ApplicationError> {
    let now = chrono::Local::now();
    let ttl = {
        if is_password_change {
            chrono::Duration::hours(1)
        } else {
            chrono::Duration::seconds(SETTINGS.secret.exp)
        }
    };
    let exp = (now + ttl).timestamp();
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
    let _: () = match redis_con.set(redis_key, String::new()) {
        Ok(result) => result,
        Err(err) => {
            return Err(ApplicationError::RedisError(err));
        }
    };
    let claims = TokenClaims { exp, uid, sid };
    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SETTINGS.secret.sec.as_bytes()),
    ) {
        Ok(token) => Ok(token),
        Err(err) => {
            return Err(ApplicationError::JwtError(err));
        }
    }
}

#[instrument(name = "Verifying confirmation token", skip(redis_con))]
pub async fn verify_confirmation_token(
    redis_con: &mut redis::Connection,
    token: String,
    is_password_change: bool,
) -> Result<ConfirmationToken, ApplicationError> {
    let claims = match decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(SETTINGS.secret.sec.as_bytes()),
        &Validation::default(),
    ) {
        Ok(token) => token.claims,
        Err(err) => {
            return Err(ApplicationError::JwtError(err));
        }
    };

    let redis_key = {
        if is_password_change {
            format!(
                "{}_password_change_{}",
                CONFIRMATION_TOKEN_PREFIX, claims.sid
            )
        } else {
            format!("{}_{}", CONFIRMATION_TOKEN_PREFIX, claims.sid)
        }
    };

    let redis_entry: Option<String> = match redis_con.get(redis_key.clone()) {
        Ok(entry) => entry,
        Err(err) => {
            return Err(ApplicationError::RedisError(err));
        }
    };

    if redis_entry.is_none() {
        return Err(ApplicationError::TokenUsed);
    }

    let _: () = match redis_con.del(redis_key.clone()) {
        Ok(result) => result,
        Err(err) => {
            return Err(ApplicationError::RedisError(err));
        }
    };

    Ok(ConfirmationToken { uid: claims.uid })
}
