use {
    crate::{
        controllers::test_controller::TestController, middleware::jwt::check_jwt, server::AppState,
    },
    axum::{
        middleware,
        routing::{get, patch},
        Router,
    },
};

pub fn get_test_router(state: &AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(TestController::get_all).post(TestController::create),
        )
        .route("/{test_id}", get(TestController::get_by_id))
        .route("/{test_id}/start", patch(TestController::start))
        .route(
            "/{test_id}/question_statuses",
            get(TestController::get_all_testing_question_statuses),
        )
        .route(
            "/{test_id}/question/{quiz_question_id}",
            get(TestController::get_testing_question).post(TestController::resolve_test_question),
        )
        .route("/{test_id}/submit", patch(TestController::submit))
        .route("/{test_id/result}", get(TestController::result))
        .route(
            "/{test_id/review/{quiz_question_id}",
            get(TestController::review_solution),
        )
        .layer(middleware::from_fn(check_jwt))
        .with_state(state.clone())
}
