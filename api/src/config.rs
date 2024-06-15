use dotenvy::dotenv;
use eyre::Result;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub(crate) struct Config {
    pub(crate) api: ApiConfig,
}

impl Config {
    pub(crate) fn load() -> Result<Self> {
        dotenv()?;
        Ok(Self {
            api: ApiConfig::load()?,
        })
    }
}

pub(crate) struct ApiConfig {
    host: [u8; 4],
    port: u16,
}

impl ApiConfig {
    pub(crate) fn load() -> Result<Self> {
        Ok(Self {
            host: std::env::var("HARBOUR_API_HOST")?
                .parse::<Ipv4Addr>()?
                .octets(),
            port: std::env::var("HARBOUR_API_PORT")?.parse::<u16>()?,
        })
    }
}

impl From<ApiConfig> for SocketAddr {
    fn from(value: ApiConfig) -> Self {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::from(value.host)), value.port)
    }
}
