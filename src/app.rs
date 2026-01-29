use axum::{Router, response::Html, routing::{get, post}};
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;

use crate::storage::StorageLayer;
use crate::routing::{public_routes, private_routes};
use crate::validation::ValidationStore;
use crate::auth::TokenService;
use crate::admin::handlers as admin_handlers;
use crate::auth::r#in as auth_in;
use crate::auth::out as auth_out;

#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<dyn StorageLayer>,
    pub validation: Arc<ValidationStore>,
    pub token_service: Arc<TokenService>,
}

pub struct AppConfig {
    pub jwt_secret: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "super-secret-key-change-in-production".to_string()),
        }
    }
}

pub async fn create_app(storage: Arc<dyn StorageLayer>) -> Router {
    create_app_with_config(storage, AppConfig::default()).await
}

pub async fn create_app_with_config(storage: Arc<dyn StorageLayer>, config: AppConfig) -> Router {
    let validation_store = Arc::new(ValidationStore::new());
    let token_service = Arc::new(TokenService::new(config.jwt_secret));

    let app_state = AppState {
        storage,
        validation: validation_store,
        token_service,
    };

    let admin_ui_routes = Router::new()
        .route("/login", get(admin_handlers::login_page).post(admin_handlers::login_submit))
        .route("/logout", post(admin_handlers::logout))
        .route("/dashboard", get(admin_handlers::dashboard))
        .route("/users", get(admin_handlers::users_list));

    let auth_routes = Router::new()
        .route("/login", post(auth_in::user_login))
        .route("/admin/login", post(auth_in::admin_login))
        .route("/logout", post(auth_out::logout))
        .route("/logout-all", post(auth_out::logout_all));

    Router::new()
        .route("/", get(root_handler))
        .merge(public_routes::router())
        .nest("/admin", admin_ui_routes)
        .nest("/admin/api", private_routes::router())
        .nest("/auth", auth_routes)
        .layer(CookieManagerLayer::new())
        .with_state(app_state)
}

async fn root_handler() -> Html<&'static str> {
    Html("<h1>Welcome but not welcome</h1>")
}
