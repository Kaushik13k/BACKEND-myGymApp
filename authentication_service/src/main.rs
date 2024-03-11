use axum::http::{ self, Method };
use axum::{ Router, ServiceExt };
use http::header::{ AUTHORIZATION, ACCEPT };

use tower::Layer;
use tower_http::cors::{ Any, CorsLayer };

mod routes;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, ACCEPT]);

    fn rewrite_request_uri<B>(req: http::Request<B>) -> http::Request<B> {
        println!("Request: {:?}", req.uri());
        println!("Request: {:?}", req.headers());
        req
    }

    let app = Router::new().nest("/api/v1", routes::health::health_check()).layer(cors);

    let middleware_creat = tower::util::MapRequestLayer::new(rewrite_request_uri);
    let app_with_middleware = middleware_creat.layer(app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app_with_middleware.into_make_service()).await.unwrap();
}
