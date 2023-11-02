use axum::{http::*, routing::*, *};
use ecdar_protobuf::services::*;
use std::result::Result;

ecdar_web_macros::add_endpoint_functions!();

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", routing::get(|| async { "Hello World" }));

    let app = ecdar_web_macros::add_endpoints!(app);

    println!("sever running on \"localhost:3000\"");
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
