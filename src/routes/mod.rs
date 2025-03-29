use {
    crate::{controllers::fallback, server::AppState},
    auth_route::get_auth_router,
    axum::{routing::get, Router},
    qna::get_question_router,
    // quiz::quiz_router,
    set::get_set_router,
    user_route::get_user_router,
};

pub mod auth_route;
mod qna;
// mod quiz;
mod set;
mod user_route;

async fn root() -> &'static str {
    "Hello, World!"
}

pub fn setup_routing(state: AppState) -> Router {
    let api_routes = Router::new()
        .nest("/auth", get_auth_router(&state))
        .nest("/users", get_user_router(&state))
        .nest("/sets", get_set_router(&state))
        .nest("/questions", get_question_router(&state));
    // .nest("/quizzes", quiz_router(&state));

    Router::new()
        .fallback(fallback)
        .route("/", get(root))
        .nest("/api", api_routes)
}
