use {
    crate::{handlers::fallback, server::AppState},
    axum::{routing::get, Router},
    set::get_set_router,
    user::get_user_router,
};

pub mod set;
pub mod user;

async fn root() -> &'static str {
    "Hello, World!"
}

pub fn setup_routing(state: AppState) -> Router {
    let api_routes = Router::new()
        .nest("/users", get_user_router(&state))
        .nest("/sets", get_set_router(&state));

    Router::new()
        .fallback(fallback)
        .route("/", get(root))
        .nest("/api", api_routes)
}
