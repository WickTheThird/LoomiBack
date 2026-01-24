use super::resp_structures::ApiResponse;
use axum::{
    routing::{get, post, put, delete},
    Router,
    Json,
    extract::State,
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;

use crate::storage::StorageLayer;

// User
async fn list_users(State(_storage): State<Arc<dyn StorageLayer>>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::<()>::not_implemented()))
}

async fn get_user(State(_storage): State<Arc<dyn StorageLayer>>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::<()>::not_implemented()))
}

async fn create_user(State(_storage): State<Arc<dyn StorageLayer>>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::<()>::not_implemented()))
}

async fn update_user(State(_storage): State<Arc<dyn StorageLayer>>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::<()>::not_implemented()))
}

async fn delete_user(State(_storage): State<Arc<dyn StorageLayer>>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::<()>::not_implemented()))   
}


// System Status
async fn system_status(State(storage): State<Arc<dyn StorageLayer>>) -> impl IntoResponse {
    let healthy = storage.health_check().await;
    if healthy {
        (StatusCode::OK, Json(ApiResponse::success("System operational")))
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Json(ApiResponse::error("System unavailable")))
    }
}


// Router
pub fn router() -> Router<Arc<dyn StorageLayer>> {
    Router::new()
        // User management > for admin
        .route("/users", get(list_users))
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))

        // System status
        .route("/system/status", get(system_status))
}
