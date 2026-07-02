use crate::{
    bootstrap::state::AppState,
    config::Config,
    infrastructure::{cache, database},
};

pub async fn build_state() -> anyhow::Result<AppState> {
    let config = Config::load()?;

    let db = database::connect(&config).await?;

    let redis = cache::redis::connect(&config).await?;

    Ok(AppState {
        config: std::sync::Arc::new(config),
        db,
        redis,
    })
}
