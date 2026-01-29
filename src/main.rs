mod app;
mod config;
mod auth;
mod users;
mod businesses;
mod websites;
mod components;
mod admin;
mod messaging;
mod storage;
mod email;
mod validation;
mod utils;
mod routing;
mod models;

use std::net::SocketAddr;
use std::sync::Arc;

use storage::MemoryStorage;

#[tokio::main]
async fn main() {
    // Create storage with a default admin for testing
    // Password: "admin123" - bcrypt hash
    let password_hash = bcrypt::hash("admin123", bcrypt::DEFAULT_COST)
        .expect("Failed to hash password");

    let storage = Arc::new(MemoryStorage::with_default_admin(
        "admin@example.com",
        &password_hash,
    ));

    println!("===========================================");
    println!("  Default admin credentials for testing:");
    println!("  Email:    admin@example.com");
    println!("  Password: admin123");
    println!("===========================================");

    let app = app::create_app(storage).await;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server running at http://{}", addr);
    println!("Admin panel: http://{}/admin/login", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
