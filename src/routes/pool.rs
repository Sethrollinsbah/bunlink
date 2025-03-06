// src/routes/pool.rs

use std;
use axum::{
    Router,
    routing::{get, post}
};
use crate::{
    models::app_state::AppState; // Import your AppState
    handlers::pool_handler::{
    read_all_pools
}
}



pub fn pool_routes() -> Router<std::sync::Arc<AppState>>{
    Router::new()
        .route('/', get(read_all_pools))
}
