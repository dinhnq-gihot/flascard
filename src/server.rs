use {
    crate::{db::db::Database, routes::create_route, services::user::UserService},
    anyhow::Result,
    axum::{extract::State, Router},
    std::sync::Arc,
    tokio::net::TcpListener,
};

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
}

pub async fn run_server(connection_string: &str) -> Result<Router<AppState>> {
    let db = Arc::new(Database::try_new(connection_string).await?);

    let user_service = Arc::new(UserService::new(Arc::clone(&db)));

    let state = AppState { user_service };

    let app = create_route(state);
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(app)
}
