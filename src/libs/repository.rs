use crate::libs::repository::RepoError::WarningNoDataFound;
use log::info;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Repo {
    storage: HashMap<String, String>,
}

pub enum RepoError {
    FailedToWrite(String),
    WarningNoDataFound(String),
}

impl Repo {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
    pub fn push(&mut self, id: &str, data: &str) {
        match self.storage.insert(id.to_string(), data.to_string()) {
            None => {
                info!("New record with id {} created", &id);
            }
            Some(data) => {
                info!("Record with id {} updated", &id);
            }
        }
    }
    pub fn pull(&self, id: &str) -> Result<String, RepoError> {
        match self.storage.get(id) {
            None => Err(WarningNoDataFound(
                "No data with provided id: {id}".to_string(),
            )),
            Some(data) => Ok(data.to_owned()),
        }
    }
}
