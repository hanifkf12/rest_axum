use crate::application::post_service::PostRepository;
use crate::domain::{
    error::PostError,
    post::{Post, PostDto},
};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

pub struct PostRepositoryImpl {
    pool: PgPool,
}

impl PostRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostRepository for PostRepositoryImpl {
    async fn get_all(&self) -> Result<Vec<Post>, PostError> {
        let posts = query_as::<_, Post>(r#"SELECT id, title, content, created_at FROM posts"#)
            .fetch_all(&self.pool)
            .await?;
        Ok(posts)
    }

    async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Post, PostError> {
        let post = query_as::<_, Post>(
            r#"SELECT id, title, content, created_at FROM posts WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| PostError::NotFound)?;

        Ok(post)
    }

    async fn create(&self, post: PostDto) -> Result<Post, PostError> {
        let post = query_as::<_, Post>(
            r#"INSERT INTO posts (id, title, content, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, title, content, created_at;"#,
        )
        .bind(Uuid::new_v4())
        .bind(post.title)
        .bind(post.content)
        .bind(Utc::now().naive_utc())
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    async fn update(&self, id: Uuid, post: PostDto) -> Result<Post, PostError> {
        let post = query_as::<_, Post>(
            r#"UPDATE posts SET title = $1, content = $2, created_at = $3 WHERE id = $4
            RETURNING id, title, content, created_at;"#,
        )
        .bind(post.title)
        .bind(post.content)
        .bind(Utc::now().naive_utc())
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    async fn delete(&self, id: Uuid) -> Result<(), PostError> {
        query(r#"DELETE FROM posts WHERE id = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
