use crate::usecases::kafka::interface::{StreamConsumerInterface, StreamProducerInterface};

pub struct StreamHandler<P: StreamProducerInterface, C: StreamConsumerInterface> {
    producer: P,
    consumer: C,
}

impl <P, C> StreamHandler<P, C>
    where P: StreamProducerInterface, C: StreamConsumerInterface {
    pub fn new(producer: P, consumer: C) -> Self {
         StreamHandler{ producer, consumer }
    }
    pub fn pull(&self) -> String {
        self.consumer.pull()
    }
}