use eyre::Result;
use reqwest::Url;

pub(crate) struct Config {
    pub(crate) api_base_url: Url,
}

impl Config {
    pub(crate) fn load() -> Result<Self> {
        Ok(Self {
            api_base_url: Url::parse("http://localhost:8080")?,
        })
    }
}
