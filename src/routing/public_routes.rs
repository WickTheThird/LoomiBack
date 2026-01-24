use super::resp_structures::{HealthResponse, HealthStatus};
use axum::{
    routing::get,
    Json,
    Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};

use std::sync::Arc;

use crate::storage::StorageLayer;


async fn health(State(storage): State<Arc<dyn StorageLayer>>) -> impl IntoResponse {
    if storage.health_check().await {
        (StatusCode::OK, Json(HealthResponse { status: HealthStatus::Ok}))
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Json(HealthResponse { status: HealthStatus::Unavailable}) )
    }
}

pub fn router() -> Router<Arc<dyn StorageLayer>> {
    Router::new().route("/health", get(health))
}
