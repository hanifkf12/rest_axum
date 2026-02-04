use crate::domain::{
    error::PostError,
    ports::PostRepository,
    post::Post,
};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

/// PostgreSQL implementation of the PostRepository port.
/// This is an infrastructure concern that adapts the database to the domain.
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
        let rows = query_as::<_, PostRow>(
            r#"SELECT id, title, content, created_at FROM posts"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| PostError::Database(e.to_string()))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Post, PostError> {
        let row = query_as::<_, PostRow>(
            r#"SELECT id, title, content, created_at FROM posts WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| PostError::Database(e.to_string()))?;
        
        match row {
            Some(r) => Ok(r.into()),
            None => Err(PostError::NotFound),
        }
    }

    async fn create(&self, title: String, content: String) -> Result<Post, PostError> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();
        
        query(
            r#"INSERT INTO posts (id, title, content, created_at)
            VALUES ($1, $2, $3, $4)"#
        )
        .bind(&id)
        .bind(&title)
        .bind(&content)
        .bind(&created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| PostError::Database(e.to_string()))?;
        
        Ok(Post::new(id, title, content, created_at))
    }

    async fn update(&self, id: Uuid, title: String, content: String) -> Result<Post, PostError> {
        let result = query(
            r#"UPDATE posts SET title = $1, content = $2 WHERE id = $3"#
        )
        .bind(&title)
        .bind(&content)
        .bind(&id)
        .execute(&self.pool)
        .await
        .map_err(|e| PostError::Database(e.to_string()))?;
        
        if result.rows_affected() == 0 {
            return Err(PostError::NotFound);
        }
        
        // Fetch the updated post to get the actual created_at
        self.get_by_id(&id).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), PostError> {
        let result = query(r#"DELETE FROM posts WHERE id = $1"#)
            .bind(&id)
            .execute(&self.pool)
            .await
            .map_err(|e| PostError::Database(e.to_string()))?;
        
        if result.rows_affected() == 0 {
            return Err(PostError::NotFound);
        }
        
        Ok(())
    }
}

/// Internal row struct for SQLx mapping.
/// This keeps the domain Post entity free from sqlx dependencies.
#[derive(sqlx::FromRow)]
struct PostRow {
    id: Uuid,
    title: String,
    content: String,
    created_at: chrono::DateTime<Utc>,
}

impl From<PostRow> for Post {
    fn from(row: PostRow) -> Self {
        Post::new(row.id, row.title, row.content, row.created_at)
    }
}
