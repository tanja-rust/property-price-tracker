mod app;
mod config;
mod cors;
mod db;
mod handlers;
mod in_models;
mod out_models;
mod repositories;
mod router;
use anyhow::Result;

#[tokio::main]

async fn main() -> Result<()> {
    app::run().await
}
