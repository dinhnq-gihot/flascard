use {
    crate::{handlers::fallback, server::AppState},
    axum::{routing::get, Router},
    qna::{get_answer_router, get_question_router},
    set::get_set_router,
    user::get_user_router,
};

mod qna;
mod set;
mod user;

async fn root() -> &'static str {
    "Hello, World!"
}

pub fn setup_routing(state: AppState) -> Router {
    let api_routes = Router::new()
        .nest("/users", get_user_router(&state))
        .nest("/sets", get_set_router(&state))
        .nest("/questions", get_question_router(&state))
        .nest("/answers", get_answer_router(&state));

    Router::new()
        .fallback(fallback)
        .route("/", get(root))
        .nest("/api", api_routes)
}
