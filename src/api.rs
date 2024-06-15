use axum::Router;
use eyre::Result;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{config::ApiConfig, database::Database};

pub(crate) struct Api {
    router: Router,
    config: ApiConfig,
}

impl Api {
    pub(crate) fn new(config: ApiConfig) -> Self {
        Self {
            router: Router::new(),
            config,
        }
    }

    pub(crate) fn initialize(mut self, router: Router) -> Self {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "api=debug,tower_http=debug,axum::rejection=trace".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();

        self.router = router.layer(TraceLayer::new_for_http());
        self
    }

    pub(crate) async fn launch(self) -> Result<()> {
        let address: SocketAddr = self.config.into();
        let listener = tokio::net::TcpListener::bind(address).await?;
        info!("HTTP server running at {}", address);
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}

pub(crate) struct ApiState {
    pub(crate) database: Database,
}
