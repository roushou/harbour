mod api;
mod config;
mod database;
mod routes;

use api::{Api, ApiState};
use config::Config;
use database::Database;
use eyre::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    let database = Database::builder().initialize().await?;
    let state = Arc::new(ApiState { database });
    Api::new(config.api, state).initialize().launch().await?;
    Ok(())
}
