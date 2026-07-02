use anyhow::Result;
use tracing::info;

use super::{dependency, migration, server};

pub async fn run() -> Result<()> {
    info!("Building application state...");
    let state = dependency::build_state().await?;

    info!("Running database migration...");
    migration::run(&state.infra.db).await?;

    info!("Starting HTTP server...");
    server::start(state).await?;

    Ok(())
}
