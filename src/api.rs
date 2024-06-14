use axum::Router;
use eyre::Result;
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{config::ApiConfig, database::Database, routes};

pub(crate) struct Api {
    router: Router,
    state: Arc<ApiState>,
    config: ApiConfig,
}

impl Api {
    pub(crate) fn new(config: ApiConfig, state: Arc<ApiState>) -> Self {
        Self {
            router: Router::new(),
            state,
            config,
        }
    }

    pub(crate) fn initialize(mut self) -> Self {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "api=debug,tower_http=debug,axum::rejection=trace".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();

        self.router = routes::router(self.state.clone()).layer(TraceLayer::new_for_http());
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
