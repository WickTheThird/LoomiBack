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

use std::env;
use std::sync::Arc;

use config::Config;
use storage::{MemoryStorage, PostgresStorage, StorageLayer};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    let storage: Arc<dyn StorageLayer> = if env::var("DATABASE_URL").is_ok() {
        match setup_postgres(&config).await {
            Ok(pg) => {
                println!("Connected to PostgreSQL database");
                Arc::new(pg)
            }
            Err(e) => {
                eprintln!("Failed to connect to database: {}", e);
                eprintln!("Falling back to in-memory storage");
                create_memory_storage()
            }
        }
    } else {
        println!("No DATABASE_URL set, using in-memory storage");
        create_memory_storage()
    };

    println!("===========================================");
    println!("  Default admin credentials for testing:");
    println!("  Email:    admin@example.com");
    println!("  Password: admin123");
    println!("===========================================");

    let app = app::create_app(storage).await;
    let addr = config.server_addr();

    println!("Server running at http://{}", addr);
    println!("Admin panel: http://{}/admin/login", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn setup_postgres(config: &Config) -> Result<PostgresStorage, storage::DbError> {
    let pg = PostgresStorage::from_url(&config.database_url).await?;
    println!("Running database migrations...");
    pg.run_migrations().await?;
    println!("Migrations completed successfully");

    let password_hash = bcrypt::hash("admin123", bcrypt::DEFAULT_COST)
        .expect("Failed to hash password");
    pg.seed_admin("admin@example.com", &password_hash).await?;

    Ok(pg)
}

fn create_memory_storage() -> Arc<dyn StorageLayer> {
    let password_hash = bcrypt::hash("admin123", bcrypt::DEFAULT_COST)
        .expect("Failed to hash password");

    Arc::new(MemoryStorage::with_default_admin(
        "admin@example.com",
        &password_hash,
    ))
}
