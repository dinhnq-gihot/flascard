use {anyhow::Result, server::run_server};

pub mod db;
pub mod entities;
pub mod error;
pub mod handlers;
pub mod routes;
pub mod server;
pub mod services;

#[tokio::main]
async fn main() -> Result<()> {
    tokio::select! {
        _ = run_server("postgres://username:password@localhost/diesel_demo") => {}
    }

    Ok(())
}
