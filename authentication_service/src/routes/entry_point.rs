use crate::database::connection::Context;
use crate::services::mutation::MutationRoot;
use crate::services::query::QueryRoot;
use axum::{extract::Json as ExtractJson, response::IntoResponse, Json};
use axum::{routing::post, Router};
use juniper::{http::GraphQLRequest, RootNode};
use std::{convert::Infallible, sync::Arc};
use tokio::task;

use log::info;

// pub struct MutationRoot;
// #[juniper::object(Context = Context)]
// impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}

pub async fn entry_point_handler(
    schema: Arc<Schema>,
    data: ExtractJson<GraphQLRequest>,
) -> impl IntoResponse {
    info!("Received a GraphQL request");

    let res = task::spawn_blocking(move || {
        let res = data.execute(&schema, &Context::new());
        Ok::<_, serde_json::Error>(serde_json::to_value(&res)?)
    })
    .await
    .unwrap();

    match res {
        Ok(user) => {
            info!("Successfully executed the GraphQL request");
            Ok::<_, Infallible>(Json(user))
        }
        Err(_) => {
            info!("Failed to execute the GraphQL request");
            Ok::<_, Infallible>(Json(serde_json::json!({"error": "Internal server error"})))
        }
    }
}

pub fn entry_point() -> Router {
    let schema = Arc::new(create_schema());

    Router::new().route(
        "/entry_point",
        post(move |data: ExtractJson<GraphQLRequest>| {
            entry_point_handler(Arc::clone(&schema), data)
        }),
    )
}
