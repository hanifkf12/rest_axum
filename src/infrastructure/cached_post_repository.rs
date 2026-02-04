use crate::domain::{
    error::PostError,
    ports::PostRepository,
    post::Post,
};
use async_trait::async_trait;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, RedisError};
use std::sync::Arc;
use tracing::warn;
use uuid::Uuid;

/// Decorator pattern implementation adding Redis caching.
/// Wraps any PostRepository implementation with caching behavior.
pub struct CachedPostRepository {
    inner: Arc<dyn PostRepository>,
    cache: MultiplexedConnection,
}

impl CachedPostRepository {
    pub fn new(inner: Arc<dyn PostRepository>, cache: MultiplexedConnection) -> Self {
        Self { inner, cache }
    }

    async fn get_cached_posts(cache: &mut MultiplexedConnection) -> Option<Vec<Post>> {
        let result: Result<String, RedisError> = cache.get("posts").await;
        match result {
            Ok(json) => match serde_json::from_str(&json) {
                Ok(posts) => Some(posts),
                Err(e) => {
                    warn!("Failed to deserialize cached posts: {}", e);
                    None
                }
            },
            Err(_) => None,
        }
    }

    async fn set_cached_posts(cache: &mut MultiplexedConnection, posts: &[Post]) {
        if let Ok(json) = serde_json::to_string(posts) {
            let _: Result<(), RedisError> = cache.set_ex("posts", json, 300).await;
        }
    }

    async fn get_cached_post(cache: &mut MultiplexedConnection, id: &Uuid) -> Option<Post> {
        let key = format!("post:{}", id);
        let result: Result<String, RedisError> = cache.get(&key).await;
        match result {
            Ok(json) => match serde_json::from_str(&json) {
                Ok(post) => Some(post),
                Err(e) => {
                    warn!("Failed to deserialize cached post {}: {}", id, e);
                    None
                }
            },
            Err(_) => None,
        }
    }

    async fn set_cached_post(cache: &mut MultiplexedConnection, post: &Post) {
        let key = format!("post:{}", post.id);
        if let Ok(json) = serde_json::to_string(post) {
            let _: Result<(), RedisError> = cache.set_ex(&key, json, 300).await;
        }
    }

    async fn invalidate_posts(cache: &mut MultiplexedConnection) {
        let _: Result<(), RedisError> = cache.del("posts").await;
    }

    async fn invalidate_post(cache: &mut MultiplexedConnection, id: &Uuid) {
        let key = format!("post:{}", id);
        let _: Result<(), RedisError> = cache.del(&key).await;
    }
}

#[async_trait]
impl PostRepository for CachedPostRepository {
    async fn get_all(&self) -> Result<Vec<Post>, PostError> {
        let mut cache = self.cache.clone();
        if let Some(posts) = Self::get_cached_posts(&mut cache).await {
            return Ok(posts);
        }
        let posts = self.inner.get_all().await?;
        Self::set_cached_posts(&mut cache, &posts).await;
        Ok(posts)
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Post, PostError> {
        let mut cache = self.cache.clone();
        if let Some(post) = Self::get_cached_post(&mut cache, id).await {
            return Ok(post);
        }
        let post = self.inner.get_by_id(id).await?;
        Self::set_cached_post(&mut cache, &post).await;
        Ok(post)
    }

    async fn create(&self, title: String, content: String) -> Result<Post, PostError> {
        let post = self.inner.create(title, content).await?;
        let mut cache = self.cache.clone();
        Self::invalidate_posts(&mut cache).await;
        Ok(post)
    }

    async fn update(&self, id: Uuid, title: String, content: String) -> Result<Post, PostError> {
        let post = self.inner.update(id, title, content).await?;
        let mut cache = self.cache.clone();
        Self::invalidate_posts(&mut cache).await;
        Self::invalidate_post(&mut cache, &id).await;
        Ok(post)
    }

    async fn delete(&self, id: Uuid) -> Result<(), PostError> {
        self.inner.delete(id).await?;
        let mut cache = self.cache.clone();
        Self::invalidate_posts(&mut cache).await;
        Self::invalidate_post(&mut cache, &id).await;
        Ok(())
    }
}
