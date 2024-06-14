mod abis;
mod health;

use axum::Router;
use std::sync::Arc;

use crate::api::ApiState;

pub(crate) fn router(state: Arc<ApiState>) -> Router {
    Router::new()
        .merge(health::router())
        .merge(abis::router(state))
}
