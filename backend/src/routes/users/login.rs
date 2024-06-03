use axum::{extract::State, http::StatusCode, response::Response, Json};
use tracing::instrument;

use crate::{app::ApplicationState, utils::response::response};

#[derive(serde::Serialize)]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
}

#[instrument(name = "Logging user in")]
pub async fn login(State(ApplicationState { .. }): State<ApplicationState>) -> Response {
    response(
        StatusCode::OK,
        None,
        Json(LoginResponse {
            access_token: "".to_owned(),
            refresh_token: "".to_owned(),
        }),
    )
}
