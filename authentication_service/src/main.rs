use axum::http::{self, Method};
use axum::{Router, ServiceExt};

use log::info;

use tower::Layer;
use tower_http::cors::{Any, CorsLayer};

mod routes;
mod utils;

#[tokio::main]
async fn main() {
    utils::logger::init_logger();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    fn rewrite_request_uri<B>(req: http::Request<B>) -> http::Request<B> {
        info!("Request: {:?}", req.uri());
        info!("Request: {:?}", req.headers());
        req
    }

    let app = Router::new()
        .nest("/api/v1", routes::health::health_check())
        .layer(cors);

    let middleware_creat = tower::util::MapRequestLayer::new(rewrite_request_uri);
    let app_with_middleware = middleware_creat.layer(app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app_with_middleware.into_make_service())
        .await
        .unwrap();
}
