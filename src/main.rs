
pub mod db;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod utils;

use axum::Router;
use crate::utils::asterisk;
use db::connection;
use dotenv::dotenv;
use models::app_state;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();

    asterisk::init().await;
    // Get server address and database URL from environment
    let server_address = std::env::var("SERVER_ADDRESS")
        .expect("SERVER_ADDRESS must be set in .env file")
        .trim_matches('"')
        .to_string();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // Setup database connection pool
    let db_pool: Pool<Postgres> = sqlx::PgPool::connect(&database_url).await?;

    // Initialize database schema
    connection::initialize_database(&db_pool).await.expect("Failed to initialize database");

    // Setup logging
    tracing_subscriber::fmt::init();

    // Create shared application state
    let shared_state = Arc::new(app_state::AppState { db_pool });

    // Build application router
    let app = Router::new()
        .nest("/status", routes::health_check::health_check_routes())
        .nest("/bid", routes::bid::bid_routes())
        // Add your routes here
        .with_state(shared_state);

    // Log server start
    println!("Server starting on address: http://{}", server_address);

    // Bind and serve
    let listener = TcpListener::bind(server_address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

