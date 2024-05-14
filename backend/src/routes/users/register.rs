use axum::extract::State;

use crate::app::ApplicationState;

pub async fn register(State(ApplicationState { .. }): State<ApplicationState>) {}
