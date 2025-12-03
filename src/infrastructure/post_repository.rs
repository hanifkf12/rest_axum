use crate::application::post_service::PostRepository;
use crate::domain::post::{Post, PostDto};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
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
    async fn get_all(&self) -> anyhow::Result<Vec<Post>> {
        let posts = sqlx::query_as!(
            Post,
            r#"SELECT id as "id!", title as "title!", content as "content!", created_at as "created_at!: _" FROM posts"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(posts)
    }

    async fn get_by_id(&self, id: &uuid::Uuid) -> anyhow::Result<Post> {
        let post = sqlx::query_as!(
            Post,
            r#"SELECT id as "id!", title as "title!", content as "content!", created_at as "created_at!: _" FROM posts WHERE id = $1"#,
            id
        ).fetch_optional(&self.pool).await?
            .ok_or_else(|| anyhow::anyhow!("Post not found"))?;

        Ok(post)
    }

    async fn create(&self, post: PostDto) -> anyhow::Result<Post> {
        let post = sqlx::query_as!(
            Post,
            r#"INSERT INTO posts (id, title, content, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id as "id!", title as "title!", content as "content!", created_at as "created_at!: _";"#,
            Uuid::new_v4(),
            post.title,
            post.content,
            Utc::now().naive_utc(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    async fn update(&self, id: Uuid, post: PostDto) -> anyhow::Result<Post> {
        todo!()
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        todo!()
    }
}
