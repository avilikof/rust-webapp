use dotenv::dotenv;
use log::error;
use std::env;

#[derive(Debug, Clone)]
pub struct StreamingSettings {
    pub bootstrap: String,
    pub username: String,
    pub password: String,
    pub default_topic: String,
    pub group_id: String,
    pub redis_address: String,
}

fn get_env(env_name: &str) -> String {
    let env_name_caps = env_name.to_owned().to_uppercase();
    match env::var(&env_name_caps) {
        Err(_) => {
            error!("Cannot get system variable: {}", &env_name_caps);
            panic!("failed to get settings from variables: {env_name_caps}");
        }
        Ok(sys_var) => sys_var,
    }
}

impl StreamingSettings {
    pub fn new() -> StreamingSettings {
        dotenv().ok();
        let bootstrap = get_env("UPSTASH_KAFKA_REST_URL");
        let username = get_env("UPSTASH_KAFKA_REST_USERNAME");
        let password = get_env("UPSTASH_KAFKA_REST_PASSWORD");
        let default_topic = get_env("UPSTASH_KAFKA_WRITE_TOPIC");
        let group_id = get_env("UPSTASH_KAFKA_GROUP_ID");
        let redis_address = get_env("UPSTASH_REDIS_ADDRESS");
        StreamingSettings {
            bootstrap,
            username,
            password,
            default_topic,
            group_id,
            redis_address,
        }
    }
}
