mod catalog;
mod intent;
mod routes;
mod summary;

use std::sync::Arc;

use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

pub struct GatewayState {
    pub query_plane_url: String,
    pub client: reqwest::Client,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let query_plane_url = std::env::var("QUERY_PLANE_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8090".into());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8091);

    tracing::info!(query_plane = %query_plane_url, port, "starting conversational-gateway");

    let state = Arc::new(GatewayState {
        query_plane_url: query_plane_url.trim_end_matches('/').to_string(),
        client: reqwest::Client::new(),
    });

    let app = Router::new()
        .merge(routes::router())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .expect("failed to bind");

    tracing::info!(port, "conversational-gateway listening");
    axum::serve(listener, app).await.expect("server error");
}
