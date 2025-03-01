use {
    crate::{
        config::Config,
        db::db::Database,
        enums::error::{Error, Result},
        error,
        routes::setup_routing,
        services::user::UserService,
    },
    std::sync::Arc,
    tokio::net::TcpListener,
    tracing::info,
};

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
}

pub async fn run_server(cfg: Config) -> Result<()> {
    let db = Arc::new(Database::try_new(&cfg.database.url).await.map_err(|e| {
        error!("{}", e.to_string());
        Error::Anyhow(e.into())
    })?);

    let user_service = Arc::new(UserService::new(Arc::clone(&db)));

    let state = AppState { user_service };

    let app = setup_routing(state);
    let listener = TcpListener::bind(format!("{}:{}", cfg.http.host, cfg.http.port))
        .await
        .map_err(|e| {
            error!("{}", e.to_string());
            Error::Anyhow(e.into())
        })?;

    info!("Server running: {}:{}", cfg.http.host, cfg.http.port);
    axum::serve(listener, app).await.map_err(|e| {
        error!("{}", e.to_string());
        Error::Anyhow(e.into())
    })?;

    Ok(())
}
