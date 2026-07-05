use anyhow::Result;
use rust_clean::infrastructure;

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = infrastructure::logging::init_logging()?;

    rust_clean::bootstrap::app::run().await
}
