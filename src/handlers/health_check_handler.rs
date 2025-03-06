
// src/handlers/health_check_handler.rs

use axum::{
    http::StatusCode,
    response::IntoResponse,
    extract::State,
};
use std::sync::Arc;
use crate::models::app_state::AppState; // Import your AppState
use sqlx::Error;

// Handler for the health check endpoint
pub async fn health_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // Check database connection
    println!("Print");
    
    match state.db_pool.acquire().await {
        Ok(conn) => {
            // Test a simple query to ensure the database is responsive.
            match sqlx::query("SELECT 1")
    .execute(&state.db_pool)
                .await
            {
                Ok(_) => (StatusCode::OK, "Server and database are healthy".to_string()),
                Err(e) => {
                    eprintln!("Database health check failed: {}", e);
                    (StatusCode::SERVICE_UNAVAILABLE, "Database check failed".to_string())
                }
            }
        }
        Err(e) => {
            eprintln!("Database connection pool error: {}", e);
            (StatusCode::SERVICE_UNAVAILABLE, "Database connection pool error".to_string())
        }
    }
}

