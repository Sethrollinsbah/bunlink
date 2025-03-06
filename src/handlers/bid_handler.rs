
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json;
use sqlx::PgPool;
use std::sync::Arc;

use crate::models::bid::{Bid, BidParams};
use crate::models::app_state::AppState; // Import your AppState

// Handler to select a bid by ID
pub async fn read_bid_by_id(
    State(state): State<Arc<AppState>>,
    Path(bid_id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let conn = &state.db_pool;

    match get_bid_by_id(conn, bid_id).await {
        Ok(bid) => Ok((StatusCode::OK, Json(bid))),
        Err(sqlx::Error::RowNotFound) => Err((StatusCode::NOT_FOUND, "Bid not found".to_string())),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )),
    }
}

async fn get_bid_by_id(conn: &PgPool, bid_id: i32) -> Result<Bid, sqlx::Error> {
    sqlx::query_as!(
        Bid,
        "SELECT id, team_id, amount, timestamp FROM public.bids WHERE id = $1",
        bid_id
    )
    .fetch_one(conn)
    .await
}

// Handler to get all bids for a team
pub async fn read_bids_by_team_id(
    State(state): State<Arc<AppState>>,
    Path(team_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let conn = &state.db_pool;

    match get_bids_by_team(conn, &team_id).await {
        Ok(bids) => Ok((StatusCode::OK, Json(bids))),
        Err(sqlx::Error::RowNotFound) => Ok((StatusCode::OK, Json(Vec::<Bid>::new()))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )),
    }
}

async fn get_bids_by_team(conn: &PgPool, team_id: &str) -> Result<Vec<Bid>, sqlx::Error> {
    sqlx::query_as!(
        Bid,
        "SELECT id, team_id, amount, timestamp FROM bids WHERE team_id = $1",
        team_id
    )
    .fetch_all(conn)
    .await
}

// Handler to create a new bid
pub async fn create_bid_by_team_id(
    State(state): State<Arc<AppState>>,
    Json(new_bid): Json<Bid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let conn = &state.db_pool;

    match insert_bid(conn, &new_bid).await {
        Ok(id) => Ok((
            StatusCode::CREATED,
            Json(Bid {
                id,
                team_id: new_bid.team_id.clone(),
                amount: new_bid.amount,
                timestamp: new_bid.timestamp,
            }),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )),
    }
}

async fn insert_bid(conn: &PgPool, new_bid: &Bid) -> Result<i32, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO bids (team_id, amount, timestamp) VALUES ($1, $2, $3) RETURNING id",
        new_bid.team_id,
        new_bid.amount,
        new_bid.timestamp
    )
    .fetch_one(conn)
    .await?;

    Ok(result.id)
}

// Handler to get all bids
pub async fn get_bids_from_db(conn: &PgPool) -> Result<Vec<Bid>, sqlx::Error> {
    sqlx::query_as!(Bid, "SELECT id, team_id, amount, timestamp FROM bids")
        .fetch_all(conn)
        .await
}

