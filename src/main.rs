use axum::{routing::get, Router};
use log::{debug, info, LevelFilter};

use std::env;


#[tokio::main]
async fn main() {
    init_logging();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/text", get(text_from_var));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn text_from_var() -> String {
    debug!("/text route executed");
    match env::var("PORT") {
        Err(e) => e.to_string(),
        Ok(var) => var
    }
}

fn init_logging() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug) // Adjust log level as needed
        .init();
}