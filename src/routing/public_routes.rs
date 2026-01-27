use super::resp_structures::{HealthResponse, HealthStatus};
use axum::{
    routing::get,
    Json,
    Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};

use crate::app::AppState;


async fn health(State(app_state): State<AppState>) -> impl IntoResponse {
    if app_state.storage.health_check().await {
        (StatusCode::OK, Json(HealthResponse { status: HealthStatus::Ok}))
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Json(HealthResponse { status: HealthStatus::Unavailable}) )
    }
}

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health))
}
