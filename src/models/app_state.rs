
use sqlx::{postgres::PgPool, Pool, Postgres};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,  // Change this to the correct Pool type (Postgres in this case)
}

