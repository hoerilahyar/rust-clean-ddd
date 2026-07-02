use sqlx::MySqlPool;

pub async fn check(pool: &MySqlPool) -> bool {
    sqlx::query("SELECT 1").execute(pool).await.is_ok()
}
