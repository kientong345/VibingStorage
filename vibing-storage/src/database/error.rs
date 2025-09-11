use serde::{Deserialize, Serialize};


pub type Result<T> = std::result::Result<T, DatabaseError>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum DatabaseError {
    QueryTimeout,
    DatabaseConnectionError,
    DatabaseError,
}

impl From<sqlx::Error> for DatabaseError {
    fn from(error: sqlx::Error) -> Self {
        // LOG_DATABASE_ERROR

        match error {
            sqlx::Error::Database(err) if err.code().as_deref() == Some("57014") => DatabaseError::QueryTimeout,
            sqlx::Error::PoolClosed | sqlx::Error::PoolTimedOut => DatabaseError::DatabaseConnectionError,
            _ => DatabaseError::DatabaseError,
        }
    }
}