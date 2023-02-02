/// A web crawler written in Rust
use anyhow::Context;
use sqlx::postgres::PgPoolOptions;

const DATABASE_URL: &str = "postgres://user:password@localhost:5432/db";

mod api;
mod crawler;
mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to the database
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(DATABASE_URL)
        .await
        .context(format!("failed to connect to {DATABASE_URL}"))?;

    // Run database migrations
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .context("failed to run migrations")?;

    // Crawl the top stories from Hacker News
    let db1 = db.clone();
    tokio::spawn(async move {
        crawler::crawl(&db1)
            .await
            .context("failed to crawl Hacker News")
            .unwrap();
    });

    // Start the HTTP server
    api::serve(db).await
}
