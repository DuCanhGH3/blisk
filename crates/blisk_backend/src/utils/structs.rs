use std::ops::{Deref, DerefMut};

use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts, Request},
    http::request::Parts,
    response::{IntoResponse, Response},
};
use axum_typed_multipart::{BaseMultipart, TryFromMultipart};
use validator::Validate;

use super::errors::AppError;

#[derive(Debug, sqlx::Type, serde::Serialize, serde::Deserialize)]
pub struct AppImage {
    id: i64,
    ext: String,
    owner: i64,
}

pub struct AppForm<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for AppForm<T>
where
    T: serde::de::DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let base = axum::Form::<T>::from_request(req, state).await?;
        base.0.validate()?;
        Ok(Self(base.0))
    }
}

pub struct AppQuery<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for AppQuery<T>
where
    T: serde::de::DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let base = axum_extra::extract::Query::<T>::from_request_parts(parts, state).await?;
        base.0.validate()?;
        Ok(Self(base.0))
    }
}

pub struct AppJson<T>(pub T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

#[async_trait]
impl<T, S> FromRequest<S> for AppJson<T>
where
    T: serde::de::DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let base = axum::Json::<T>::from_request(req, state).await?;
        base.0.validate()?;
        Ok(Self(base.0))
    }
}

#[derive(Debug)]
pub struct AppMultipart<T>(pub T);

impl<T> Deref for AppMultipart<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for AppMultipart<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait]
impl<T, S> FromRequest<S> for AppMultipart<T>
where
    T: TryFromMultipart + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let base = BaseMultipart::<T, Self::Rejection>::from_request(req, state).await?;
        base.validate()?;
        Ok(Self(base.data))
    }
}
