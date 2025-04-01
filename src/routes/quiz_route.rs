use {
    super::quiz_question_route::quiz_question_router,
    crate::{
        controllers::quiz_controller::QuizController, middleware::jwt::check_jwt, server::AppState,
    },
    axum::{
        middleware,
        routing::{get, post},
        Router,
    },
};

pub fn quiz_router(state: &AppState) -> Router {
    let quiz_router = Router::new()
        .route("/", get(QuizController::get_all))
        .route(
            "/{id}",
            get(QuizController::get_one)
                .post(QuizController::create)
                .patch(QuizController::update)
                .delete(QuizController::delete),
        )
        .layer(middleware::from_fn(check_jwt))
        .with_state(state.clone());

    let share_quiz_router = Router::new()
        .route("/{id}/share", post(QuizController::share))
        .route(
            "/{id}/shared_users",
            get(QuizController::get_all_shared_users_of_quiz),
        )
        .layer(middleware::from_fn(check_jwt))
        .with_state(state.clone());

    // let quiz_question_router: Router = Router::new()
    //     .route(
    //         "/{id}/questions",
    //         post(QuizQuestionHandler::create).get(QuizQuestionHandler::get_all),
    //     )
    //     .route(
    //         "/{quiz_id}/questions/{quiz_question_id}",
    //         get(QuizQuestionHandler::get_by_id)
    //             .patch(QuizQuestionHandler::update)
    //             .delete(QuizQuestionHandler::delete),
    //     )
    //     .layer(middleware::from_fn(check_jwt))
    //     .with_state(state.clone());

    Router::new()
        .merge(quiz_router)
        .merge(share_quiz_router)
        .merge(quiz_question_router(state))
    // .merge(quiz_question_router)
}
