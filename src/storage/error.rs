use std::fmt;

#[derive(Debug)]
pub enum DbError {
    NotFound,
    Duplicate(String),
    Connection(String),
    Query(String),
    Migration(String),
    Other(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::NotFound => write!(f, "Record not found"),
            DbError::Duplicate(field) => write!(f, "Duplicate value for field: {}", field),
            DbError::Connection(msg) => write!(f, "Database connection error: {}", msg),
            DbError::Query(msg) => write!(f, "Query error: {}", msg),
            DbError::Migration(msg) => write!(f, "Migration error: {}", msg),
            DbError::Other(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for DbError {}

impl From<sqlx::Error> for DbError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => DbError::NotFound,
            sqlx::Error::Database(db_err) => {
                // Check for unique constraint violations
                if db_err.code().map(|c| c == "23505").unwrap_or(false) {
                    DbError::Duplicate(db_err.message().to_string())
                } else {
                    DbError::Query(db_err.message().to_string())
                }
            }
            sqlx::Error::Io(io_err) => DbError::Connection(io_err.to_string()),
            _ => DbError::Other(err.to_string()),
        }
    }
}
