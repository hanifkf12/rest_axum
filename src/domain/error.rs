use sqlx::Error as SqlxError;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum PostError {
    NotFound,
    Database(SqlxError),
}

impl Display for PostError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            PostError::NotFound => write!(f, "post not found"),
            PostError::Database(err) => write!(f, "database error: {err}"),
        }
    }
}

impl std::error::Error for PostError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PostError::Database(err) => Some(err),
            _ => None,
        }
    }
}

impl From<SqlxError> for PostError {
    fn from(err: SqlxError) -> Self {
        match err {
            SqlxError::RowNotFound => PostError::NotFound,
            other => PostError::Database(other),
        }
    }
}
