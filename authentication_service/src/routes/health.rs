use axum::{ routing::get, Router };

async fn health() -> &'static str {
    "Health Check OK"
}
pub fn health_check() -> Router {
    Router::new().route("/health", get(health))
}
