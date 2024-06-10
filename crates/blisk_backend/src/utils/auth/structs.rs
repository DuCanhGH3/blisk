use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::{settings::SETTINGS, utils::errors::AppError};

use super::errors::AuthError;

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, serde::Deserialize, serde::Serialize)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
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
    pub is_verified: Option<bool>,
    pub role: Option<UserRole>,
    pub password: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserClaims {
    pub iss: String,
    pub sub: i64,
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
}

impl UserClaims {
    pub fn encode(&self) -> Result<String, AppError> {
        Ok(encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(SETTINGS.auth.access.sec.as_bytes()),
        )?)
    }
    pub fn decode(token: &str) -> Result<UserClaims, AppError> {
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_audience(&["abc"]);
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

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::Invalid)?;
        Ok(UserClaims::decode(bearer.token())?)
    }
}
