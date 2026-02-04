use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Domain entity representing a blog post.
/// This is a pure domain object with no external dependencies.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl Post {
    pub fn new(id: Uuid, title: String, content: String, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            title,
            content,
            created_at,
        }
    }
}
