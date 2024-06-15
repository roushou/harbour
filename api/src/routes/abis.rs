use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{api::ApiState, database::abis::Abi};

pub(crate) fn router(state: Arc<ApiState>) -> Router {
    async fn get_by_id(
        Path(id): Path<i64>,
        State(state): State<Arc<ApiState>>,
    ) -> Result<Json<Abi>, StatusCode> {
        let tx = state.database.abis.get_by_id(id).await.unwrap();
        match tx {
            Some(abi) => Ok(Json(abi)),
            None => Err(StatusCode::NOT_FOUND),
        }
    }

    async fn list(State(state): State<Arc<ApiState>>) -> Result<Json<Vec<Abi>>, StatusCode> {
        let tx = state.database.abis.list().await;
        match tx {
            Ok(abis) => Ok(Json(abis)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    #[derive(Deserialize)]
    struct CreateAbi {
        name: String,
    }

    #[derive(Serialize)]
    struct CreateAbiResponse {
        id: i64,
    }

    async fn create(
        State(state): State<Arc<ApiState>>,
        Json(payload): Json<CreateAbi>,
    ) -> Result<Json<CreateAbiResponse>, StatusCode> {
        let tx = state.database.abis.create(payload.name).await;
        match tx {
            Ok(id) => Ok(Json(CreateAbiResponse { id })),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    Router::new()
        .route("/abis", get(list).post(create))
        .route("/abis/:id", get(get_by_id))
        .with_state(state)
}
