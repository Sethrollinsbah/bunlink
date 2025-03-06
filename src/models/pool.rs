// src/models/pool.rs

use serde::{Deserialize, Serialize};
use crate::{
    models::{
        us_state::UsState
    }
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pool {
    pub id: i64,
    pub name: String,
    pub description: Option<String>, // Optional description
    pub created_at: i64, // Unix timestamp of creation
    pub is_active: bool, // Indicates if the pool is currently active
    pub owner_id: i64, // ID of the user or entity that created the pool
    pub location_id: i64, // optional location of the pool
    pub max_participants: Option<i32>, //Optional maximum number of participants.
}
