use axum::{
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};

#[derive(serde::Serialize)]
pub struct SuccessResponse {
    pub message: String,
}
#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn response<T>(
    status: StatusCode,
    headers: Option<Vec<(HeaderName, HeaderValue)>>,
    json: Json<T>,
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
