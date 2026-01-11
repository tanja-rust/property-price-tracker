use crate::app::state::AppState;
use crate::app::state::build_app_state;
use crate::config::AppConfig;
use crate::cors::get_cors_layer;
use crate::router::build_router;
use anyhow::{Context, Result};

use std::net::SocketAddr;

use tracing::info;

pub fn load_config() -> Result<AppConfig> {
    dotenvy::dotenv().ok();
    AppConfig::load().context("Failed to load AppConfig")
}

pub async fn init_state(config: &AppConfig) -> Result<AppState> {
    build_app_state(config).await
}

pub async fn start_http_server(app_state: AppState) -> Result<()> {
    let app = build_router(app_state.clone()).layer(get_cors_layer());

    let addr = SocketAddr::from((
        app_state.config.app_host.parse::<std::net::IpAddr>()?,
        app_state.config.app_port,
    ));
    info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("Failed to bind to address")?;

    axum::serve(listener, app)
        .await
        .context("HTTP server error")
}
