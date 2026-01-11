pub mod bootstrap;
pub mod logging;
pub mod middleware;
pub mod state;

use anyhow::Result;

pub async fn run() -> Result<()> {
    // 1. Load config
    let config = bootstrap::load_config()?;

    // 2. Init logging
    logging::init_logging(&config)?;

    // 3. Init app state (DB, repos, clients…)
    let app_state = bootstrap::init_state(&config).await?;

    // 4. Start HTTP server
    bootstrap::start_http_server(app_state).await?;

    Ok(())
}
