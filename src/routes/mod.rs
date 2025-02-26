use {
    crate::server::AppState,
    axum::{routing::get, Router},
    user::get_user_router,
};

pub mod user;

async fn root() -> &'static str {
    "Hello, World!"
}

pub fn create_route(state: AppState) -> Router<AppState> {
    let api_routes = Router::new().nest("/user", get_user_router());
    Router::new()
        .route("/", get(root))
        .nest("/api", api_routes)
        .with_state(state)
}
