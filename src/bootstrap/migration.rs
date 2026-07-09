use anyhow::Result;
use sqlx::MySqlPool;

/// Intentionally a no-op in the running application.
///
/// The SQL files under `./migrations` are written as `DROP TABLE IF EXISTS`
/// followed by `CREATE TABLE` — they reset state rather than incrementally
/// altering it. Wiring this into real app startup would drop and recreate
/// every table (destroying data) on each restart, so it stays a no-op here.
/// Migrations are expected to be applied manually / via a controlled
/// deploy step.
///
/// For tests, use `test_support::migration::run_from_dir`, which runs the
/// same files against a disposable database (e.g. a testcontainer) where
/// that destructive behavior is exactly what's wanted.
pub async fn run(_db: &MySqlPool) -> Result<()> {
    Ok(())
}
