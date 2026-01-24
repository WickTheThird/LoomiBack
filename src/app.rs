use axum::{
    Router,
    response::Html,
    routing::get,
};
use std::sync::Arc;

use crate::storage::StorageLayer;
use crate::routing::{private_routes, public_routes};


pub async fn create_app(storage: Arc<dyn StorageLayer>) -> Router {


    Router::new()
        .route("/", get(root_handler))
        .merge(public_routes::router())
        .nest("/admin", private_routes::router())
        .with_state(storage)
}

async fn root_handler() -> Html<&'static str> {
    Html("<h1>Welcome but not welcome</h1>")
}
