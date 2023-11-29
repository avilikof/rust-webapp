use crate::libs::repository::Repo;
use crate::usecases::kafka_db::error::KafkaDbHandlerError;

struct KafkaDbHandler {
    database: Repo,
}

impl KafkaDbHandler {
    pub fn new(repo: Repo) -> Self {
        Self { database: repo }
    }

    pub fn push(&mut self, id: &str, data: &str) -> Result<(), KafkaDbHandlerError> {
        if !self.data_exists(id) {
            self.database.push(id, data)
        }
        Err(KafkaDbHandlerError::Failed(
            "Data exists with id: {id}".to_string(),
        ))
    }
    pub fn pull(&self, id: &str) -> Result<String, KafkaDbHandlerError> {
        match self.database.pull(id) {
            Err(e) => Err(KafkaDbHandlerError::Failed(
                "No data for id: {id}".to_string(),
            )),
            Ok(data) => Ok(data.to_owned()),
        }
    }
    fn data_exists(&self, id: &str) -> bool {
        match self.pull(id) {
            Err(_) => false,
            Ok(_) => true,
        }
    }
}
