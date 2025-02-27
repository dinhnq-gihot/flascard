use {
    crate::{handlers::user::*, server::AppState},
    axum::{
        routing::{get, post},
        Router,
    },
};

pub fn get_user_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(UserHandler::get_all_users))
        .route("/register", post(UserHandler::register_user))
        .with_state(state)
}
