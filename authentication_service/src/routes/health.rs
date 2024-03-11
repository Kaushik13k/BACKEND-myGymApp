use axum::{routing::get, Router};
use log::info;

async fn health() -> &'static str {
    info!("Executing Health API...");
    "Health Check OK"
}
pub fn health_check() -> Router {
    Router::new().route("/health", get(health))
}
