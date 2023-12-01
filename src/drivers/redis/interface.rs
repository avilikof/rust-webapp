pub trait RedisSettings {
    fn redis_address(&self) -> String;
}
