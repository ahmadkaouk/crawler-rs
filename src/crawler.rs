use futures::{stream, StreamExt};
use sqlx::PgPool;

use crate::{
    api::post::{HackerNewsPost, Post},
    error::Error,
};

/// Get Hacker News top stories ids
pub async fn get_top_stories_ids() -> anyhow::Result<Vec<u32>> {
    let response = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .await?
        .text()
        .await?;

    let top_stories: Vec<u32> = serde_json::from_str(&response)?;
    Ok(top_stories)
}

/// Persist a post in the database
pub async fn persist_post(db: &PgPool, post: Post) -> Result<(), Error> {
    sqlx::query(
            r#"INSERT INTO posts (id, time, url, title, author, score) VALUES ($1, $2, $3, $4, $5, $6)"#,
        )
        .bind(post.id)
        .bind(post.time)
        .bind(&post.url)
        .bind(&post.title)
        .bind(&post.author)
        .bind(post.score)
        .execute(db)
        .await?;
    Ok(())
}

/// Get the top stories from Hacker News and store them in the database
pub async fn crawl(db: &PgPool) -> anyhow::Result<()> {
    let top_stories: Vec<u32> = get_top_stories_ids().await?;
    stream::iter(top_stories)
        .map(|id| async move {
            let url = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json?print=pretty");
            // Parse the JSON response into a Post struct
            let post: HackerNewsPost = reqwest::get(&url).await?.json().await?;
            persist_post(db, post.into()).await?;
            Ok::<(), anyhow::Error>(())
        })
        .buffer_unordered(10)
        .collect::<Vec<_>>()
        .await;
    Ok(())
}
