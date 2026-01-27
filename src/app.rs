use axum::{
    Router,
    response::Html,
    routing::get,
};
use std::sync::Arc;

use crate::storage::StorageLayer;
use crate::routing::{public_routes, private_routes};
use crate::validation::ValidationStore;


#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<dyn StorageLayer>,
    pub validation: Arc<ValidationStore>,
}

pub async fn create_app(storage: Arc<dyn StorageLayer>) -> Router {

    let validation_store = Arc::new(ValidationStore::new());

    let app_state = AppState {
        storage,
        validation: validation_store,
    };

    Router::new()
        .route("/", get(root_handler))
        .merge(public_routes::router())
        .nest("/admin", private_routes::router())
        .nest("/admin/api", private_routes::router())
        .with_state(app_state)
}

async fn root_handler() -> Html<&'static str> {
    Html("<h1>Welcome but not welcome</h1>")
}
