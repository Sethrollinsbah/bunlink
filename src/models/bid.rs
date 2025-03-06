// src/models/bid.rs

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Bid {
    pub id: i32,
    pub team_id: String,
    pub amount: f32,
    pub timestamp: i64,
}

impl Bid {
    // Convert to response (if you need special response handling)
    pub fn to_response(&self) -> Self {
        self.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct BidParams {
    pub id: Option<i32>,
    pub team_id: Option<String>,
    pub amount: Option<f32>,
    pub timestamp: Option<i64>,
}

impl BidParams {
    // Create a new set of parameters for insertion
    pub fn for_insert(team_id: String, amount: f32, timestamp: i64) -> Self {
        Self {
            id: None,
            team_id: Some(team_id),
            amount: Some(amount),
            timestamp: Some(timestamp),
        }
    }
    
    // Create a new set of parameters for querying
    pub fn for_query() -> Self {
        Self::default()
    }
    
    // Create a new Bid from valid parameters
    pub fn to_bid(&self) -> Option<Bid> {
        match (self.id, &self.team_id, self.amount, self.timestamp) {
            (Some(id), Some(team_id), Some(amount), Some(timestamp)) => Some(Bid {
                id,
                team_id: team_id.clone(),
                amount,
                timestamp,
            }),
            _ => None,
        }
    }
    
    // Convert valid parameters to a new Bid (for insertion)
    pub fn to_new_bid(&self) -> Option<Bid> {
        match (&self.team_id, self.amount, self.timestamp) {
            (Some(team_id), Some(amount), Some(timestamp)) => Some(Bid {
                id: 0, // This would be replaced by the database
                team_id: team_id.clone(),
                amount,
                timestamp,
            }),
            _ => None,
        }
    }
}

// use serde::{Deserialize, Serialize};
//
// #[derive(Deserialize, Serialize, Clone, Debug)]
// pub struct Bid {
//     pub id: i32,
//     pub team_id: String,
//     pub amount: f64,
//     pub timestamp: i64,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug)]
// pub struct BidInsert {
//     pub team_id: Option<String>,
//     pub amount: Option<f64>,
//     pub timestamp: Option<i64>, // Added timestamp field
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug)]
// pub struct NewBid {
//     pub team_id: String,
//     pub amount: f64,
//     pub timestamp: i64,
// }
//
// #[derive(Deserialize, Serialize)]
// pub struct BidQuery {
//     pub id: Option<i32>,
//     pub team_id: Option<String>,
//     pub timestamp: Option<i64>,
// }
//
// #[derive(Serialize)]
// pub struct BidResponse {
//     pub id: i32,
//     pub team_id: String,
//     pub amount: f64,
//     pub timestamp: i64,
// }
