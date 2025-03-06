// src/routes/bid.rs

use axum::{Router, routing::{get, post}};
use std::sync::Arc;
use crate::{
    models::app_state::AppState,
    handlers::bid_handler::{
        read_bid_by_id, read_bids_by_team_id,  
        create_bid_by_team_id
    }
};

pub fn bid_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{id}", get(read_bid_by_id)) // Get bid by ID
        .route("/", post(create_bid_by_team_id)) // Create a new bid
        .route("/team/{id}", get(read_bids_by_team_id)) // Get bid by ID
        // .route("/, ", )
}
