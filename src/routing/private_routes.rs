use super::resp_structures::ApiResponse;
use axum::{
    routing::{get, post, put, delete},
    Router,
    Json,
    extract::State,
    response::IntoResponse,
    http::StatusCode,
};

use crate::app::AppState;


// User
async fn list_users(State(_app_state): State<AppState>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::<()>::not_implemented()))
}

async fn get_user(State(_app_state): State<AppState>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::<()>::not_implemented()))
}

async fn create_user(State(_app_state): State<AppState>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::<()>::not_implemented()))
}

async fn update_user(State(_app_state): State<AppState>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::<()>::not_implemented()))
}

async fn delete_user(State(_app_state): State<AppState>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, Json(ApiResponse::<()>::not_implemented()))   
}


// System Status
async fn system_status(State(app_stae): State<AppState>) -> impl IntoResponse {
    let healthy = app_stae.storage.health_check().await;
    if healthy {
        (StatusCode::OK, Json(ApiResponse::success("System operational")))
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Json(ApiResponse::error("System unavailable")))
    }
}


// Router
pub fn router() -> Router<AppState> {
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
