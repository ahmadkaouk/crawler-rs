use crate::post::Post;
use anyhow::{Context, Ok};
use sqlx::{postgres::PgPoolOptions, PgPool};


#[derive(Debug, Clone)]
pub struct Db {
    pool: PgPool,
}

impl Db {
    /// Create a new database connection pool
    pub async fn new() -> anyhow::Result<Self> {

        // Create the posts table if it doesn't exist
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS posts (
                id INTEGER PRIMARY KEY,
                time INTEGER,
                url TEXT,
                title TEXT,
                author TEXT,
                score INTEGER
            )"#,
        )
        .execute(&pool)
        .await?;

        Ok(Db { pool })
    }

    /// Get a reference to the database connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn insert_post(&self, post: Post) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO posts (id, time, url, title, author, score) VALUES ($1, $2, $3, $4, $5, $6)"#,
        )
        .bind(post.id)
        .bind(post.time)
        .bind(&post.url)
        .bind(&post.title)
        .bind(&post.author)
        .bind(post.score)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_new() {
        let db = Db::new().await.unwrap();
        assert_eq!(db.pool().size(), 1);
    }
}
