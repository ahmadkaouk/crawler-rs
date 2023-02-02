use anyhow::Context;
use axum::Router;
use sqlx::PgPool;

pub mod post;

/// Create the router
pub fn app(db: PgPool) -> Router {
    Router::new().merge(post::router()).with_state(db)
}

pub async fn serve(db: PgPool) -> anyhow::Result<()> {
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app(db).into_make_service())
        .await
        .context("failed to start server")
}
