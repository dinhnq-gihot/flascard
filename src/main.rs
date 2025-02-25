use {
    anyhow::Result,
    axum::{routing::get, Router},
    tokio::net::TcpListener,
};

pub mod db;
pub mod entities;
pub mod error;
pub mod handlers;
pub mod routes;
pub mod services;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    let app = Router::new().route("/", get(|| async { "Hello, Axum!" }));

    axum::serve(listener, app).await?;

    Ok(())
}
