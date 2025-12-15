use crate::domain::{
    error::PostError,
    post::{Post, PostDto},
};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Post>, PostError>;
    async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Post, PostError>;
    async fn create(&self, post: PostDto) -> Result<Post, PostError>;
    async fn update(&self, id: uuid::Uuid, post: PostDto) -> Result<Post, PostError>;
    async fn delete(&self, id: uuid::Uuid) -> Result<(), PostError>;
}

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

    pub async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Post, PostError> {
        self.repo.get_by_id(id).await
    }

    pub async fn create(&self, post: PostDto) -> Result<Post, PostError> {
        self.repo.create(post).await
    }

    pub async fn update(&self, id: uuid::Uuid, post: PostDto) -> Result<Post, PostError> {
        self.repo.update(id, post).await
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<(), PostError> {
        self.repo.delete(id).await
    }
}
