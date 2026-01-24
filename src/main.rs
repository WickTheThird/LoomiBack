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

use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let storage = Arc::new(storage::MemoryStorage::new());
    
    let app = app::create_app(storage).await;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
