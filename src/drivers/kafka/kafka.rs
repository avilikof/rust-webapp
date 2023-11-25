use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::Consumer;
use rdkafka::error::KafkaError;
use rdkafka::message::OwnedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use log::debug;
use crate::drivers::kafka::interface::KafkaSettings;

#[derive(Debug, Clone)]
pub struct StreamingKafka <S: KafkaSettings> {
    consumer_config: ClientConfig,
    streaming_settings: S,
}

impl<S: KafkaSettings> StreamingKafka<S> {
    pub fn new(s: S) -> Self {
        let mut consumer_config = ClientConfig::new();
        consumer_config
            .set("bootstrap.servers", s.bootstrap())
            .set("security.protocol", "SASL_SSL")
            .set("sasl.mechanisms", "SCRAM-SHA-256")
            .set("sasl.username", s.username())
            .set("sasl.password", s.password())
            .set("group.id", s.group_id())
            .set("auto.offset.reset", "latest")
            .set("ssl.ca.location", "/usr/lib/aarch64-linux-gnu");
        Self {
            consumer_config,
            streaming_settings: s,
        }
    }

    pub fn subscribe(&mut self) -> StreamConsumer {
        let consumer: StreamConsumer = self
            .consumer_config
            .create()
            .expect("Consumer creation failed");

        consumer
            .subscribe(&[&self.streaming_settings.default_topic()])
            .expect("Subscription failed");
        consumer
    }

    pub async fn produce(
        &mut self,
        data: &Vec<u8>,
    ) -> Result<(i32, i64), (KafkaError, OwnedMessage)> {
        let producer: &FutureProducer = &self
            .consumer_config
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation failed");

        let write_topic = self.streaming_settings.default_topic();
        let record = FutureRecord::to(write_topic.as_str())
            .key("key1")
            .payload(data);

        // Return result from below code
        match producer.send(record, Duration::from_secs(0)).await {
            Err(e) => {
                log::error!("{:?}", e);
                Err(e)
            }
            Ok(result) => {
                debug!("Message produced");
                Ok(result)
            }
        }
    }
}
