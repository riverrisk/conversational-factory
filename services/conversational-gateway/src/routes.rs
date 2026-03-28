use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};

use cf_shared::gateway::*;

use crate::catalog;
use crate::intent;
use crate::GatewayState;

type SharedState = Arc<GatewayState>;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/health", get(health))
        .route("/tools", get(tools))
        .route("/query", post(query))
}

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok", "service": "conversational-gateway" }))
}

async fn tools() -> impl IntoResponse {
    Json(catalog::tool_catalog())
}

async fn query(
    State(state): State<SharedState>,
    Json(request): Json<GatewayRequest>,
) -> impl IntoResponse {
    let response = intent::handle_request(&state, request).await;
    Json(response)
}
