use crate::usecases::kafka::interface::{StreamConsumerInterface, StreamProducerInterface};
use log::{debug, error};
use std::sync::mpsc::SyncSender;

pub struct StreamHandler<P: StreamProducerInterface, C: StreamConsumerInterface> {
    producer: P,
    consumer: C,
    channel: SyncSender<String>,
}

impl<P, C> StreamHandler<P, C>
where
    P: StreamProducerInterface,
    C: StreamConsumerInterface,
{
    pub fn new(producer: P, consumer: C, channel: SyncSender<String>) -> Self {
        StreamHandler {
            producer,
            consumer,
            channel,
        }
    }
    pub fn pull(&mut self) -> String {
        self.consumer.pull()
    }
    fn fill_buffer(&mut self) {
        loop {
            match self.channel.send(self.consumer.pull()) {
                Err(e) => {
                    let err_as_string = e.to_string();
                    error!("failed sending to buffer {}", err_as_string);
                }
                Ok(_) => debug!("Message sent to buffer"),
            }
        }
    }
}
