use axum::{
    http::{header, HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use validator::ValidationErrors;

use super::structs::AppJson;

#[derive(serde::Serialize)]
pub struct SuccessResponse {
    pub message: String,
}
#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
#[derive(serde::Serialize)]
pub struct ValidationErrorResponse {
    pub validation_error: ValidationErrors,
}

pub fn empty(headers: Option<Vec<(HeaderName, HeaderValue)>>) -> Response {
    let mut res = StatusCode::NO_CONTENT.into_response();
    if let Some(headers) = headers {
        *res.headers_mut() = HeaderMap::from_iter(headers);
    }
    res
}

pub fn created(location: String) -> Response {
    let mut res = StatusCode::CREATED.into_response();
    res.headers_mut().insert(header::LOCATION, header::HeaderValue::from_str(&location).unwrap());
    res
}

pub fn response<T>(
    status: StatusCode,
    headers: Option<Vec<(HeaderName, HeaderValue)>>,
    json: AppJson<T>,
) -> Response
where
    T: serde::Serialize,
{
    let mut res = (status, json).into_response();
    if let Some(headers) = headers {
        *res.headers_mut() = HeaderMap::from_iter(headers);
    }
    res
}
