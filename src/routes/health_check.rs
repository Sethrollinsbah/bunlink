// src/routes/health_check.rs

use axum::{Router, routing::get};
use std::sync::Arc; // Import your AppState
use crate::{
    models::app_state::AppState,
    handlers::health_check_handler::health_check
};

// Define your health check route
pub fn health_check_routes() -> Router<Arc<AppState>> {
    Router::new().route("/health", get(health_check))
}
