use crate::drivers::kafka::interface::KafkaSettingsInterface;
use crate::libs::settings::StreamingSettings;

impl KafkaSettingsInterface for StreamingSettings {
    fn bootstrap(&self) -> String {
        self.bootstrap.clone()
    }

    fn username(&self) -> String {
        self.username.clone()
    }

    fn password(&self) -> String {
        self.password.clone()
    }

    fn group_id(&self) -> String {
        self.group_id.clone()
    }

    fn default_topic(&self) -> String {
        self.default_topic.clone()
    }
}
