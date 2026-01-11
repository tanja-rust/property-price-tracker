use crate::config::AppConfig;
use anyhow::Result;

pub fn init_logging(config: &AppConfig) -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(&config.rust_log)
        .json()
        .init();

    Ok(())
}
