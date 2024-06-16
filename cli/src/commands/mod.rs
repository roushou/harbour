mod abi;

use clap::{Parser, Subcommand};
use eyre::Result;
use reqwest::Url;

use self::abi::AbiCommand;

#[derive(Parser)]
#[command(name = "harbour")]
#[command(version, about)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub(crate) async fn run(base_url: Url) -> Result<()> {
        let cli = Cli::parse();
        match cli.command {
            Command::Abi(abi) => abi.run(base_url).await?,
        };
        Ok(())
    }
}

#[derive(Subcommand)]
enum Command {
    #[command(arg_required_else_help = true)]
    Abi(AbiCommand),
}
