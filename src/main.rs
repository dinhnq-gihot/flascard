use {anyhow::Result, server::run_server};

pub mod db;
pub mod entities;
pub mod enums;
pub mod handlers;
pub mod logger;
pub mod models;
pub mod routes;
pub mod server;
pub mod services;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init(None, true)?;

    tokio::select! {
        _ = run_server("postgres://username:password@localhost/diesel_demo") => {}
    }

    Ok(())
}
