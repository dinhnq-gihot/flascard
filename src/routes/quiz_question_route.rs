use {
    crate::{
        controllers::quiz_question_controller::QuizQuestionController, middleware::jwt::check_jwt,
        server::AppState,
    },
    axum::{
        middleware,
        routing::{get, post},
        Router,
    },
};

pub fn quiz_question_router(state: &AppState) -> Router {
    Router::new()
        .route(
            "/{quiz_id}/questions",
            post(QuizQuestionController::create)
                .get(QuizQuestionController::get_all)
                .patch(QuizQuestionController::update),
        )
        .route(
            "/{quiz_id}/questions/{quiz_question_id}",
            get(QuizQuestionController::get_by_id).delete(QuizQuestionController::delete),
        )
        .layer(middleware::from_fn(check_jwt))
        .with_state(state.clone())
}
