use {
    anyhow::Result,
    clap::Parser,
    config::{Cli, Config},
    r#static::{init_blacklist_jwt, write_blacklist_jwt},
    server::run_server,
    std::fs,
};

pub mod config;
pub mod controllers;
pub mod db;
pub mod entities;
pub mod enums;
pub mod logger;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod server;
pub mod services;
pub mod r#static;
pub mod tests;
pub mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init(None, true)?;

    // load environment variables from a .env file
    dotenv::dotenv().ok();

    let args = Cli::parse();
    let toml_file = fs::read_to_string(args.cfg)?;
    let cfg = Config::from_cfg(&toml_file)?;

    init_blacklist_jwt(&cfg.jwt_blacklist.path)?;

    tokio::select! {
        res = run_server(cfg.clone()) => {error!("Server stopped unexpectedly"); res?;},
        res = write_blacklist_jwt(&cfg.jwt_blacklist.path) => res?
    }

    Ok(())
}
