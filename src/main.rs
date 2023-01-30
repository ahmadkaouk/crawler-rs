/// A web crawler written in Rust
use anyhow::Ok;
use axum::{routing::get, Router};
use db::Db;
use futures::{stream, StreamExt};
use post::{create_post, posts, HackerNewsPost, top_posts, posts_by_user};

mod db;
mod error;
mod post;

/// Get the top stories from Hacker News and store them in the database
pub async fn crawl(db: &Db) -> anyhow::Result<()> {
    // Get the ids of top stories from Hacker News
    let response = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .await?
        .text()
        .await?;
    let top_stories: Vec<u32> = serde_json::from_str(&response)?;
    stream::iter(top_stories)
        .map(|id| async move {
            let url = format!(
                "https://hacker-news.firebaseio.com/v0/item/{}.json?print=pretty",
                id
            );
            // Parse the JSON response into a Post struct
            let post: HackerNewsPost = reqwest::get(&url).await?.json().await?;
            db.insert_post(post.into()).await?;
            Ok(())
        })
        .buffer_unordered(10)
        .collect::<Vec<_>>()
        .await;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = Db::new().await?;

    let db1 = db.clone();
    // Crawl the top stories from Hacker News
    tokio::spawn(async move {
            crawl(&db1).await.unwrap();
    });

    let app = Router::new()
        .route("/post/:id", get(post::post).post(create_post))
        .route("/posts", get(posts))
        .route("/posts/top", get(top_posts))
        .route("/posts/:user", get(posts_by_user))
        .with_state(db);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
