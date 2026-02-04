use crate::domain::{
    error::PostError,
    ports::PostRepository,
    post::Post,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

/// Application-level DTO for creating/updating posts.
/// This is separate from domain entities to allow flexibility.
#[derive(Debug, Deserialize, Clone)]
pub struct CreatePostDto {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdatePostDto {
    pub title: String,
    pub content: String,
}

/// Application service implementing use cases.
/// Orchestrates domain logic and coordinates repositories.
pub struct PostService {
    repo: Arc<dyn PostRepository>,
}

impl PostService {
    pub fn new(repo: Arc<dyn PostRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Post>, PostError> {
        self.repo.get_all().await
    }

    pub async fn get_by_id(&self, id: &Uuid) -> Result<Post, PostError> {
        self.repo.get_by_id(id).await
    }

    pub async fn create(&self, dto: CreatePostDto) -> Result<Post, PostError> {
        // Business validation can be added here
        if dto.title.is_empty() {
            return Err(PostError::InvalidInput("Title cannot be empty".to_string()));
        }
        self.repo.create(dto.title, dto.content).await
    }

    pub async fn update(&self, id: Uuid, dto: UpdatePostDto) -> Result<Post, PostError> {
        // Business validation can be added here
        if dto.title.is_empty() {
            return Err(PostError::InvalidInput("Title cannot be empty".to_string()));
        }
        self.repo.update(id, dto.title, dto.content).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), PostError> {
        self.repo.delete(id).await
    }
}
