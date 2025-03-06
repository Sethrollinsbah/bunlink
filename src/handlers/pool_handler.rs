// src/handlers/pool_handler.rs

use crate::models::pool::Pool;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use sqlx::{postgres::PgPool, Error};
use std::sync::Arc;
use crate::models::app_state::AppState;
use axum::extract::{State, Path};

pub async fn read_pool_by_id(
    State(state): State<Arc<AppState>>,
    Path(pool_id): Path<i64>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let pool = state.db_pool.clone();
    
    match get_pool_by_id(&pool, pool_id).await {
        Ok(pool) => Ok((StatusCode::OK, Json(pool))),
        Err(e) => match e {
            Error::RowNotFound => {
                Err((StatusCode::NOT_FOUND, "Pool not found".to_string()))
            }
            _ => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))),
        },
    }
}



async fn get_pool_by_id(pool: &PgPool, pool_id: i64) -> Result<Pool, Error> {
let pool = sqlx::query!(
    r#"
    SELECT id, name, description, created_at, is_active, owner_id, location_id, max_participants 
    FROM pools 
    WHERE id = $1
    "#,
    pool_id as i64
)
.map(|row| Pool {
    id: row.id as i64,
    name: row.name,
    description: row.description,
    created_at: row.created_at,
    is_active: row.is_active.expect("Is active is missing"),
    owner_id: row.owner_id as i64,
    location_id: row.location_id.unwrap_or(0) as i64, // handle null values
    max_participants: row.max_participants,
})
.fetch_one(pool)
.await?;    Ok(pool)
}

