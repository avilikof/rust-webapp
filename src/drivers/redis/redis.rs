use crate::drivers::redis::interface::RedisSettings;
use log::{debug, info};
use redis::Commands;
use std::time::Instant;

pub async fn call_redis<S: RedisSettings>(settings: S, key: Option<&str>, data: Option<&str>) {
    // match data {
    //     None => {
    //         let i = fetch_an_integer().unwrap();
    //         info!("{}", i);
    //     }
    //     Some(data) => {
    //         let i = fetch_string(data).unwrap();
    //         info!("{}", i);
    //     }
    // };
    // // let i = fetch_an_integer().unwrap();
    fetch_string(settings, key, data).unwrap();
}

fn fetch_an_integer<S: RedisSettings>(settings: S) -> redis::RedisResult<isize> {
    let client = redis::Client::open(settings.redis_address())?;
    let mut con = client.get_connection()?;

    // throw away the result, just make sure it does not fail
    con.set("my_key", 42)?;

    con.get("my_key")
}

fn fetch_string<S: RedisSettings>(
    settings: S,
    key: Option<&str>,
    data: Option<&str>,
) -> redis::RedisResult<String> {
    let mut the_key = "some_key";
    let mut the_data = "some data";
    let start = Instant::now();

    match key {
        None => {}
        Some(d) => the_key = d,
    }
    match data {
        None => {}
        Some(d) => the_data = d,
    }

    let client_time = Instant::now();
    let client = redis::Client::open(settings.redis_address())?;
    debug!("Client creation time: {:?}", client_time.elapsed());

    let connection_time = Instant::now();
    let mut con = client.get_connection()?;
    debug!("Connection creation time: {:?}", connection_time.elapsed());

    let set_time = Instant::now();
    con.set(the_key, the_data)?;
    debug!("Set operation time: {:?}", set_time.elapsed());

    let get_time = Instant::now();
    let result = con.get(the_key);
    debug!("Get operation time: {:?}", get_time.elapsed());

    info!("Total function execution time: {:?}", start.elapsed());

    result
}
