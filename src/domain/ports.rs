use crate::domain::{error::PostError, post::Post};
use async_trait::async_trait;
use uuid::Uuid;

/// Repository trait defining the contract for post persistence.
/// This is a domain port - infrastructure implements this interface.
#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Post>, PostError>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Post, PostError>;
    async fn create(&self, title: String, content: String) -> Result<Post, PostError>;
    async fn update(&self, id: Uuid, title: String, content: String) -> Result<Post, PostError>;
    async fn delete(&self, id: Uuid) -> Result<(), PostError>;
}
