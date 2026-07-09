use std::net::SocketAddr;

use anyhow::Result;
use tokio::net::TcpListener;

use super::{router::create_router, state::AppState};

pub async fn start(state: AppState) -> Result<()> {
    let host = state.config.app.host.clone();
    let port = state.config.app.port;

    let listener = TcpListener::bind(format!("{host}:{port}")).await?;
    tracing::info!("Server running on http://{}:{}", host, port);

    let server = axum::serve(
        listener,
        create_router(state).into_make_service_with_connect_info::<SocketAddr>(),
    );

    let shutdown = async {
        tokio::signal::ctrl_c().await.ok();
        tracing::info!("Shutdown signal received");
    };

    server.with_graceful_shutdown(shutdown).await?;
    Ok(())
}
