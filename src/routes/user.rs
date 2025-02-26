use {
    crate::{handlers::user::*, server::AppState},
    axum::{routing::get, Router},
};

pub fn get_user_router() -> Router<AppState> {
    Router::new().route("/", get(get_all_users))
}
