use serde::{Deserialize, Serialize};


pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum AppError {
    AudioTagError(String),
}

impl From<audiotags::Error> for AppError {
    fn from(error: audiotags::Error) -> Self {
        // LOG_DATABASE_ERROR
        println!("{:?}", error);

        match error {
            audiotags::Error::ReadError { source } => AppError::AudioTagError(source.to_string()),
            _ => AppError::AudioTagError(String::new()),
        }
    }
}