use {
    crate::{
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

pub async fn run_server(connection_string: &str) -> Result<()> {
    let db = Arc::new(Database::try_new(connection_string).await.map_err(|e| {
        error!("{}", e.to_string());
        Error::Anyhow(e.into())
    })?);

    let user_service = Arc::new(UserService::new(Arc::clone(&db)));

    let state = AppState { user_service };

    let app = setup_routing(state);
    let listener = TcpListener::bind("0.0.0.0:3000").await.map_err(|e| {
        error!("{}", e.to_string());
        Error::Anyhow(e.into())
    })?;

    info!("Server running: 0.0.0.0:3000");
    axum::serve(listener, app).await.map_err(|e| {
        error!("{}", e.to_string());
        Error::Anyhow(e.into())
    })?;

    Ok(())
}
