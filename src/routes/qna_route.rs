use {
    crate::{
        controllers::qna_controller::QnAController, middleware::jwt::check_jwt, server::AppState,
    },
    axum::{middleware, routing::get, Router},
};

pub fn get_question_router(state: &AppState) -> Router {
    Router::new()
        .route("/", get(QnAController::get_all).post(QnAController::create))
        .route(
            "/{id}",
            get(QnAController::get_by_id)
                .patch(QnAController::update)
                .delete(QnAController::delete),
        )
        .layer(middleware::from_fn(check_jwt))
        .with_state(state.clone())
}
