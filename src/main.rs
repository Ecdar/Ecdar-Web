use axum::{routing::*, *};
use clap::Parser;
use ecdar_web::{add_endpoint_functions, add_endpoints};
use std::env::current_dir;
use std::path::PathBuf;
use std::result::Result;
use tower_http::services::ServeDir;

add_endpoint_functions!();

#[tokio::main] // TODO: Maybe add some testing eventually
async fn main() -> Result<(), impl std::error::Error> {
    let args = Args::parse();
    let file_service = get_service(ServeDir::new(args.root));

    let app = Router::new().nest_service("/", file_service);

    let app = add_endpoints!(app);

    println!(r#"server running on \"{}\""#, args.serve);
    Server::bind(&args.serve.parse().unwrap())
        .serve(app.into_make_service())
        .await
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about="TODO: Describe", long_about = None)]
pub struct Args {
    #[arg(short, long, value_name = "PATH", default_value = current_dir().unwrap().into_os_string())]
    root: PathBuf,

    #[arg(short, long, value_name = "IP:PORT", default_value_t = String::from("0.0.0.0:3000"))]
    serve: String,
}
