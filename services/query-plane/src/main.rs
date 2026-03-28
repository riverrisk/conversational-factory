mod routes;
mod sample;
mod upstream;

use std::sync::Arc;

use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let upstream_url = std::env::var("SEMANTIC_DNS_URL").ok();
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8090);

    let provider: Arc<dyn routes::AssetProvider> = match upstream_url {
        Some(url) => {
            tracing::info!(url = %url, "connecting to semantic-dns upstream");
            Arc::new(upstream::UpstreamProvider::new(url))
        }
        None => {
            tracing::info!("no SEMANTIC_DNS_URL set, running with sample data");
            Arc::new(sample::SampleProvider::new())
        }
    };

    let state = Arc::new(routes::QueryPlaneState { provider });

    let app = Router::new()
        .merge(routes::router())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .expect("failed to bind");

    tracing::info!(port, "query-plane listening");
    axum::serve(listener, app).await.expect("server error");
}
