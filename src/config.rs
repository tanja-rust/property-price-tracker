use anyhow::{Context, Result};
use std::env;
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub skip_migrations: bool,
    pub api_secret_key: String,
    pub app_host: String,
    pub app_port: u16,
    pub rust_log: String,
}

impl AppConfig {
    /// Loads the application configuration from environment variables.
    pub fn load() -> Result<Self> {
        // Required / secret variables
        let database_url = required_env("DATABASE_URL");
        let api_secret_key = required_env("API_SECRET_KEY");

        let app_host = optional_env("APP_HOST", "0.0.0.0");
        let app_port: u16 = optional_env("APP_PORT", "8081")
            .parse()
            .context("APP_PORT must be a number")?;
        println!("Database URL: {}", database_url);
        let skip_migrations: bool = optional_env("SKIP_MIGRATIONS", "false") == "true";
        let rust_log = optional_env("RUST_LOG", "info,sqlx=warn,sqlx::query=warn");

        Ok(Self {
            database_url,
            skip_migrations,
            api_secret_key,
            rust_log,
            app_host,
            app_port,
        })
    }
}

fn required_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Environment variable {} must be set", key))
}

fn optional_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database_url: "postgres://user:pass@localhost/db".to_string(),
            skip_migrations: true,
            api_secret_key: "default_secret_key".to_string(),
            app_host: "0.0.0.0".to_string(),
            app_port: 8081,
            rust_log: "info,sqlx=warn,sqlx::query=warn".to_string(),
        }
    }
}
