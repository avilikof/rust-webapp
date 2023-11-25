use axum::{routing::get, Json, Router};
use log::{debug, LevelFilter};

use serde::Serialize;
use std::{
    env,
    time::{Duration, Instant},
};

#[derive(Serialize)]
struct ExecTime {
    response: String,
    time: Duration,
}

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

async fn text_from_var() -> Json<ExecTime> {
    debug!("/text route executed");
    // match env::var("PORT") {
    //     Err(e) => e.to_string(),
    //     Ok(var) => var,
    // }
    let start_time = Instant::now();
    let env_var = get_env("PORT");
    let end_time = Instant::now();
    let duration = end_time - start_time;
    Json(execution_report(duration, &env_var))
}

fn init_logging() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug) // Adjust log level as needed
        .init();
}

fn execution_time<F, S>(func: F) -> (Duration, String)
where
    F: FnOnce() -> std::string::String,
{
    let start_time = Instant::now();
    let func_resp: String = func();
    let end_time = Instant::now();
    (end_time - start_time, func_resp)
}

fn get_env(env_name: &str) -> String {
    let env_name_caps = env_name.to_owned().to_uppercase();
    match env::var(env_name_caps) {
        Err(e) => e.to_string(),
        Ok(syst_var) => syst_var,
    }
}

fn execution_report(duration: Duration, result: &str) -> ExecTime {
    ExecTime {
        time: duration,
        response: result.to_owned(),
    }
}
