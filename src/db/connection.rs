
use sqlx::{Pool, Postgres, Executor};
use std::fs;

pub async fn initialize_database(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let schema = fs::read_to_string("./schema.sql").expect("Failed to read schema file");
    pool.execute(schema.as_str()).await?;
    Ok(())
}

