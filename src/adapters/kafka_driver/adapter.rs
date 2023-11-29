// use crate::drivers::kafka::interface::KafkaSettingsInterface;
// use crate::drivers::kafka::kafka::StreamingKafka;
// use crate::usecases::kafka::interface::StreamConsumerInterface;
// use rdkafka::consumer::Consumer;
//
// impl<S: KafkaSettingsInterface> StreamConsumerInterface for StreamingKafka<S> {
//     async fn pull(&mut self) -> String {
//         let kafka_client = self.subscribe();
//         let consumer = &kafka_client.subscribe("write");
//         let a = consumer.recv().await.unwrap().payload().unwrap().to_vec();
//         match String::from_utf8(a) {
//             Err(_) => "Failed to convert from byte to string".to_string(),
//             Ok(res) => res,
//         }
//     }
// }
