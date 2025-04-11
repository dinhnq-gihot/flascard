use {
    crate::{controllers::fallback, server::AppState},
    auth_route::get_auth_router,
    axum::{routing::get, Router},
    qna_route::get_question_router,
    quiz_route::quiz_router,
    set_route::get_set_router,
    test_route::get_test_router,
    user_route::get_user_router,
};

pub mod auth_route;
mod qna_route;
pub mod quiz_question_route;
mod quiz_route;
mod set_route;
pub mod test_route;
mod user_route;

async fn root() -> &'static str {
    "Hello, World!"
}

pub fn setup_routing(state: AppState) -> Router {
    let api_routes = Router::new()
        .nest("/auth", get_auth_router(&state))
        .nest("/users", get_user_router(&state))
        .nest("/sets", get_set_router(&state))
        .nest("/questions", get_question_router(&state))
        .nest("/quizzes", quiz_router(&state))
        .nest("/test", get_test_router(&state));

    Router::new()
        .fallback(fallback)
        .route("/", get(root))
        .nest("/api", api_routes)
}
