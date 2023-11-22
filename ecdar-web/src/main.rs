pub mod configuration;
use configuration::Configuration;

use axum::{http::*, routing::*, *};
use std::result::Result;
use tower_http::services::ServeDir;

ecdar_web_macros::add_endpoint_functions!();

#[tokio::main]
async fn main() {
    let conf = Configuration::create();
    dbg!(&conf);

    let file_service = get_service(ServeDir::new(conf.root));

    let app = Router::new().nest_service("/", file_service);

    let app = ecdar_web_macros::add_endpoints!(app);

    println!(r#"sever running on "{}""#, conf.serve);
    Server::bind(&conf.serve.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
