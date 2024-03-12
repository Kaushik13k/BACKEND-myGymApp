use axum::http;
use std::vec;

use log::info;

pub fn set_context<B>(request: http::Request<B>) -> http::Request<B> {
    let request_uri = request.uri().path().to_string();
    info!("Request: {:?}", request_uri);
    info!("Request: {:?}", request.headers());
    let new_uri_path = if request_uri.starts_with("/api/v1") {
        request_uri.replacen("/api/v1", "", 1)
    } else {
        return request;
    };

    let new_uri_path_str: &str = &new_uri_path;

    if vec!["/health", "/signup", "/token"].contains(&new_uri_path_str) {
        info!("new_uri_path is in the defined_strings vector");
    } else {
        info!("new_uri_path is not in the defined_strings vector");
    }

    // my_dict.insert("key", "value");
    request
}
