use anyhow::{Result, anyhow};
use sqlx::MySqlPool;

pub async fn check_database(db: &MySqlPool) -> Result<()> {
    sqlx::query("SELECT 1")
        .fetch_one(db)
        .await
        .map_err(|e| anyhow!("Database health check failed: {}", e))?;
    Ok(())
}
