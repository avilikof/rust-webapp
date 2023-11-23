use axum::{routing::get, Router};
use std::env;


#[tokio::main]
async fn main() {
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
    match env::var("PORT") {
        Err(e) => e.to_string(),
        Ok(var) => var
    }
}
