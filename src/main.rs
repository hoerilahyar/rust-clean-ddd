use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    rust_clean::middleware::logging::init();

    rust_clean::bootstrap::app::run().await
}
