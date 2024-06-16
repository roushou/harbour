use clap::{Args, Subcommand};
use eyre::Result;
use reqwest::Url;
use serde::Deserialize;

#[derive(Args)]
pub(crate) struct AbiCommand {
    #[command(subcommand)]
    command: AbiSubcommand,
}

impl AbiCommand {
    pub(crate) async fn run(self, base_url: Url) -> Result<()> {
        let result = self.command.run(base_url).await?;
        println!("{:?}", result);
        Ok(())
    }
}

#[derive(Subcommand)]
enum AbiSubcommand {
    Get { id: u64 },
    List,
    Push,
}

impl AbiSubcommand {
    async fn run(self, base_url: Url) -> Result<AbiSubcommandResult> {
        let result = match self {
            Self::Get { id } => {
                let path = format!("/abis/{}", id);
                let url = base_url.join(path.as_str())?;
                let response = reqwest::get(url).await?.json::<Abi>().await?;
                AbiSubcommandResult::Get(response)
            }
            Self::List => {
                let url = base_url.join("/abis")?;
                let response = reqwest::get(url).await?.json::<Vec<Abi>>().await?;
                AbiSubcommandResult::List(response)
            }
            Self::Push => {
                let url = base_url.join("/abis")?;
                let client = reqwest::Client::new();
                let result = client
                    .post(url)
                    .send()
                    .await?
                    .json::<CreateAbiResponse>()
                    .await?;
                AbiSubcommandResult::Push(result)
            }
        };
        Ok(result)
    }
}

#[derive(Debug)]
enum AbiSubcommandResult {
    Get(Abi),
    List(Vec<Abi>),
    Push(CreateAbiResponse),
}

#[derive(Debug, Deserialize)]
struct Abi {
    id: i64,
    name: String,
}

#[derive(Debug, Deserialize)]
struct CreateAbiResponse {
    pub(crate) id: i64,
}
