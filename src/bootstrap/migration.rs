use anyhow::Result;
use sqlx::MySqlPool;

pub async fn run(_db: &MySqlPool) -> Result<()> {
    Ok(())
}
