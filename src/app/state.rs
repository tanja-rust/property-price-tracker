use crate::config::AppConfig;
use crate::db::AppDatabase;
use anyhow::{Context, Result};
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: AppConfig,
}

pub async fn build_app_state(config: &AppConfig) -> Result<AppState> {
    // DB
    let app_db = AppDatabase::init(config)
        .await
        .context("Failed to initialize database")?;

    Ok(AppState {
        db: app_db.db.clone(),
        config: config.clone(),
    })
}
