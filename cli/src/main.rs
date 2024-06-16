mod commands;
mod config;

use commands::Cli;
use config::Config;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    Cli::run(config.api_base_url).await?;
    Ok(())
}
