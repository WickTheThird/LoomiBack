use axum::{
    routing::get,
    Router,
    response::Html,
};
use std::sync::Arc;

use crate::storage::StorageLayer;


pub async fn create_app(storage: Arc<dyn StorageLayer>) -> Router {


    Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_check))
        .with_state(storage)
}

async fn root_handler() -> Html<&'static str> {
    Html("<h1>Welcome but not welcome</h1>")
}

async fn health_check() -> &'static str {
    "OK"
}
