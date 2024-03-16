use crate::database::connection::Context;
use axum::http;
use log::info;
use std::vec;

use crate::{services::get_user::get_availablity, utils::token::validate_token};

pub fn set_context<B>(request: http::Request<B>) -> http::Request<B> {
    let request_uri = request.uri().path().to_string();
    info!("Request: {:?}", request_uri);
    info!("Request: {:?}", request.headers());
    let operation_type = request
        .headers()
        .get("operation-type")
        .unwrap()
        .to_str()
        .unwrap()
        .to_lowercase();

    if vec!["health", "signup", "token"].contains(&operation_type.as_str()) {
        info!(
            "operation_type is {:?}, Hence no Auth needed for this request.",
            operation_type
        );
        request
    } else {
        let auth_header = request.headers().get("Authorization");
        if auth_header.is_none() {
            panic!("Auth is needed for this request. Not in headers.");
        } else {
            info!(
                "operation_type is {:?}, Auth is needed for this request.",
                operation_type
            );
            let auth_token = request
                .headers()
                .get("Authorization")
                .unwrap()
                .to_str()
                .unwrap()
                .split("Bearer ")
                .collect::<Vec<&str>>()[1];
            if auth_token.is_empty() {
                panic!("Auth is needed for this request.");
            } else {
                let validate_token = validate_token(auth_token.to_string());
                info!("Token is validated: {:?}", validate_token);
                if validate_token.is_none() {
                    panic!("Token is not valid. Could not validate.");
                } else {
                    info!("Token is validated.");
                    let context = Context::new();
                    let user_availablity = get_availablity(&context, validate_token.unwrap());
                    info!("User availablity: {:?}", user_availablity);
                    match user_availablity {
                        Ok(user_availablity) => {
                            info!("User availablity: {:?}", user_availablity);
                            if user_availablity {
                                info!("User is available.");
                            } else {
                                panic!("User is not available.");
                            }
                        }
                        Err(_) => panic!("User is not available."),
                    }
                }
            }
        }
        request
    }
}
