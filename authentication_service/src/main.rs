use axum::Router;
use axum::{http::Method, ServiceExt};

use tower::Layer;
use tower_http::cors::{Any, CorsLayer};

mod database;
mod middleware;
mod routes;
mod schema;
mod services;
mod utils;
mod models;

#[tokio::main]
async fn main() {
    utils::logger::init_logger();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app = Router::new()
        .nest("/api/v1", routes::health::health_check())
        .nest("/api/v1", routes::entry_point::entry_point())
        .layer(cors);

    let token_middleware = tower::util::MapRequestLayer::new(middleware::middleware::set_context);
    let token_middleware = token_middleware.layer(app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, token_middleware.into_make_service())
        .await
        .unwrap();
}
