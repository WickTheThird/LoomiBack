use super::resp_structures;
use axum::{routing::get, Json, Router};


async fn health() -> Json<resp_structures::HealthResponse> {
    Json(resp_structures::HealthResponse { status: "ok" })
}

pub fn router() -> Router {
    Router::new().route("/health", get(health))
}
