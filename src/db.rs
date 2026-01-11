use crate::config::AppConfig;
use anyhow::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use tracing::info;
#[derive(Clone)]
pub struct AppDatabase {
    pub db: DatabaseConnection,
}

impl AppDatabase {
    pub async fn init(config: &AppConfig) -> Result<Self> {
        info!(
            "💽 Connecting to Property price tracker DB...{}",
            config.database_url
        );
        let db = Database::connect(&config.database_url).await?;
        info!("✅ Connected to DB!");

        if !config.skip_migrations {
            Migrator::up(&db, None)
                .await
                .expect("Failed to run migrations on startup");
        }

        Ok(Self { db })
    }
}
