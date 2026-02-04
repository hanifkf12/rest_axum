use std::fmt::{Display, Formatter, Result as FmtResult};

/// Domain errors representing business rule violations and system failures.
#[derive(Debug)]
pub enum PostError {
    NotFound,
    InvalidInput(String),
    Database(String),
    Cache(String),
}

impl Display for PostError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            PostError::NotFound => write!(f, "post not found"),
            PostError::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
            PostError::Database(msg) => write!(f, "database error: {msg}"),
            PostError::Cache(msg) => write!(f, "cache error: {msg}"),
        }
    }
}

impl std::error::Error for PostError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
