use crate::drivers::redis::interface::RedisSettings;
use crate::libs::settings::StreamingSettings;

impl RedisSettings for StreamingSettings {
    fn redis_address(&self) -> String {
        self.redis_address.clone()
    }
}
