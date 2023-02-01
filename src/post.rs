use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{db::Db, error::Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct HackerNewsPost {
    pub(crate) by: String,
    pub(crate) descendants: Option<i32>,
    pub(crate) id: i32,
    pub(crate) kids: Option<Vec<i32>>,
    pub(crate) score: i32,
    pub(crate) time: i32,
    #[serde(rename = "type")]
    pub(crate) type_: String,
    pub(crate) url: Option<String>,
    pub(crate) title: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub(crate) author: String,
    pub(crate) id: i32,
    pub(crate) score: i32,
    pub(crate) time: i32,
    pub(crate) url: Option<String>,
    pub(crate) title: String,
}

impl From<HackerNewsPost> for Post {
    fn from(post: HackerNewsPost) -> Self {
        Post {
            author: post.by,
            id: post.id,
            score: post.score,
            time: post.time,
            url: post.url,
            title: post.title,
        }
    }
}

pub async fn create_post(State(db): State<Db>, Json(req): Json<Post>) -> Result<(), Error> {
    sqlx::query(
            r#"INSERT INTO posts (id, time, url, title, author, score) VALUES ($1, $2, $3, $4, $5, $6)"#,
        )
        .bind(&req.id.to_string())
        .bind(&req.time.to_string())
        .bind(&req.url)
        .bind(&req.title)
        .bind(&req.author)
        .bind(&req.score.to_string())
        .execute(db.pool())
        .await?;
    Ok(())
}

pub async fn post(State(db): State<Db>, Path(id): Path<i32>) -> Result<Json<Post>, Error> {
    let post = sqlx::query_as(r#"SELECT * FROM posts WHERE id = $1"#)
        .bind(id)
        .fetch_one(db.pool())
        .await?;
    Ok(Json(post))
}

pub async fn posts(State(db): State<Db>) -> Result<Json<Vec<Post>>, Error> {
    let posts = sqlx::query_as(r#"SELECT * FROM posts"#)
        .fetch_all(db.pool())
        .await?;
    Ok(Json(posts))
}

pub async fn top_posts(State(db): State<Db>) -> Result<Json<Vec<Post>>, Error> {
    let posts = sqlx::query_as(r#"SELECT * FROM posts ORDER BY score DESC LIMIT 20"#)
        .fetch_all(db.pool())
        .await?;
    Ok(Json(posts))
}

pub async fn posts_by_user(
    State(db): State<Db>,
    Path(user): Path<String>,
) -> Result<Json<Vec<Post>>, Error> {
    let posts = sqlx::query_as(r#"SELECT * FROM posts WHERE author = $1"#)
        .bind(user)
        .fetch_all(db.pool())
        .await?;
    Ok(Json(posts))
}
