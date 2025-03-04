use {
    crate::{handlers::qna::*, middleware::jwt::check_jwt, server::AppState},
    axum::{
        middleware,
        routing::{get, patch},
        Router,
    },
};

pub fn get_question_router(state: &AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(QnAHandler::get_all).post(QnAHandler::create_question),
        )
        .route(
            "/{id}",
            get(QnAHandler::get_by_id)
                .patch(QnAHandler::update_question)
                .delete(QnAHandler::delete_question),
        )
        .layer(middleware::from_fn(check_jwt))
        .with_state(state.clone())
}

pub fn get_answer_router(state: &AppState) -> Router {
    Router::new()
        .route(
            "/{id}",
            patch(QnAHandler::update_answer).delete(QnAHandler::delete_answer),
        )
        .layer(middleware::from_fn(check_jwt))
        .with_state(state.clone())
}
