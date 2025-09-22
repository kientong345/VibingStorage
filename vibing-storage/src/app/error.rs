use crate::database::error::DatabaseError;
use serde::{Deserialize, Serialize};
use tokio::io;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum AppError {
    AudioTagError(String),
    DatabaseError(String),
    IoError(String),
}

impl From<audiotags::Error> for AppError {
    fn from(error: audiotags::Error) -> Self {
        // LOG_AUDIO_ERROR

        match error {
            audiotags::Error::ReadError { source } => AppError::AudioTagError(source.to_string()),
            _ => AppError::AudioTagError(String::new()),
        }
    }
}

impl From<DatabaseError> for AppError {
    fn from(_error: DatabaseError) -> Self {
        // LOG_DATABASE_ERROR

        AppError::DatabaseError(String::from(""))
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        // LOG_IO_ERROR

        AppError::IoError(error.to_string())
    }
}
