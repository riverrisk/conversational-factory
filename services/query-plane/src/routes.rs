use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;

use cf_shared::assets::SemanticRecord;
use cf_shared::query::RecordFilter;
use cf_shared::system::SyncStatus;

pub trait AssetProvider: Send + Sync + 'static {
    fn resolve(
        &self,
        target: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<SemanticRecord>, String>> + Send + '_>>;

    fn query(
        &self,
        filter: &RecordFilter,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<SemanticRecord>, String>> + Send + '_>>;

    fn sync_status(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<SyncStatus, String>> + Send + '_>>;
}

pub struct QueryPlaneState {
    pub provider: Arc<dyn AssetProvider>,
}

type SharedState = Arc<QueryPlaneState>;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/health", get(health))
        .route("/assets/resolve", get(resolve))
        .route("/assets/query", get(query_assets))
        .route("/sync-status", get(sync_status))
}

#[derive(Deserialize)]
struct ResolveParams {
    target: String,
}

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok", "service": "query-plane" }))
}

async fn resolve(
    State(state): State<SharedState>,
    Query(params): Query<ResolveParams>,
) -> impl IntoResponse {
    match state.provider.resolve(&params.target).await {
        Ok(Some(record)) => Json(serde_json::json!(record)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(serde_json::json!({
            "error": "not-found",
            "target": params.target
        }))).into_response(),
        Err(e) => (StatusCode::BAD_GATEWAY, Json(serde_json::json!({
            "error": "upstream-error",
            "message": e
        }))).into_response(),
    }
}

async fn query_assets(
    State(state): State<SharedState>,
    Query(filter): Query<RecordFilter>,
) -> impl IntoResponse {
    match state.provider.query(&filter).await {
        Ok(records) => Json(serde_json::json!(records)).into_response(),
        Err(e) => (StatusCode::BAD_GATEWAY, Json(serde_json::json!({
            "error": "upstream-error",
            "message": e
        }))).into_response(),
    }
}

async fn sync_status(
    State(state): State<SharedState>,
) -> impl IntoResponse {
    match state.provider.sync_status().await {
        Ok(status) => Json(serde_json::json!(status)).into_response(),
        Err(e) => (StatusCode::BAD_GATEWAY, Json(serde_json::json!({
            "error": "upstream-error",
            "message": e
        }))).into_response(),
    }
}
