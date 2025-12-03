use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::post::{Post, PostDto};

#[async_trait]
pub trait PostRepository : Send + Sync {
    async fn get_all(&self) -> anyhow::Result<Vec<Post>>;
    async fn get_by_id(&self, id: &uuid::Uuid) -> anyhow::Result<Post>;
    async fn create(&self, post: PostDto) -> anyhow::Result<Post>;
    async fn update(&self, id: uuid::Uuid, post: PostDto) -> anyhow::Result<Post>;
    async fn delete(&self, id: uuid::Uuid) -> anyhow::Result<()>;
}


pub struct PostService {
    repo: Arc<dyn PostRepository>
}

impl PostService {
    pub fn new(repo: Arc<dyn PostRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> anyhow::Result<Vec<Post>> {
        self.repo.get_all().await
    }

    pub async fn get_by_id(&self, id: &uuid::Uuid) -> anyhow::Result<Post> {
        self.repo.get_by_id(id).await
    }

    pub async fn create(&self, post: PostDto) -> anyhow::Result<Post> {
        self.repo.create(post).await
    }

    pub async fn update(&self, id: uuid::Uuid, post: PostDto) -> anyhow::Result<Post> {
        self.repo.update(id, post).await
    }
}
