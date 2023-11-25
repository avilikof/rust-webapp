pub trait KafkaSettings {
    fn bootstrap(&self) -> String;
    fn username(&self) -> String;
    fn password(&self) -> String;
    fn group_id(&self) -> String;
    fn default_topic(&self) -> String;
}