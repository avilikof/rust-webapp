mod adapters;
mod drivers;
mod libs;
mod usecases;

use axum::{routing::get, Json, Router};
use log::{debug, LevelFilter};

use crate::drivers::kafka::kafka::StreamingKafka;
use crate::libs::repository::Repo;
use crate::libs::settings::StreamingSettings;
use rdkafka::Message;
use serde::Serialize;
use std::sync::Arc;
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
    let mut repo = Repo::new();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/text", get(text_from_var))
        .route("/kafka", get(|| kafka(&mut repo)));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn text_from_var() -> Json<ExecTime> {
    debug!("/text route executed");
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

async fn execution_time<F, Fut>(func: F) -> (Duration, String)
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = String>,
{
    let start_time = Instant::now();
    let fut = func();
    let func_resp = fut.await;
    let end_time = Instant::now();
    (end_time - start_time, func_resp)
}

fn get_env(env_name: &str) -> String {
    let env_name_caps = env_name.to_owned().to_uppercase();
    match env::var(env_name_caps) {
        Err(e) => e.to_string(),
        Ok(sys_var) => sys_var,
    }
}

fn execution_report(duration: Duration, result: &str) -> ExecTime {
    ExecTime {
        time: duration,
        response: result.to_owned(),
    }
}

async fn read_kafka_message(repo: &mut Repo) -> String {
    let kafka_settings = StreamingSettings::new();
    let mut kafka_client = StreamingKafka::new(kafka_settings);
    let consumer = &kafka_client.subscribe();
    let a = consumer.recv().await.unwrap().payload().unwrap().to_vec();
    match String::from_utf8(a) {
        Err(_) => "Failed to convert from byte to string".to_string(),
        Ok(res) => {
            repo.push("a", &res);
            res
        }
    }
}

async fn kafka(repo: &mut Repo) -> Json<ExecTime> {
    let resp = execution_time(|| read_kafka_message(repo)).await;
    Json(execution_report(resp.0, &resp.1))
}
