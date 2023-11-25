use crate::usecases::kafka::error::StreamHandlerError;

pub trait StreamConsumerInterface {
    fn pull(&self) -> String;
}

pub trait StreamProducerInterface {
    fn push(&self, msg: &str) -> Result<(), StreamHandlerError>;
}