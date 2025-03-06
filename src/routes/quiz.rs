use {
    crate::{
        handlers::{
            quiz::QuizHandler, quiz_question::QuizQuestionHandler, share_quiz::ShareQuizHandler,
        },
        middleware::jwt::check_jwt,
        server::AppState,
    },
    axum::{
        middleware,
        routing::{get, post},
        Router,
    },
};

pub fn quiz_router(state: &AppState) -> Router {
    let quiz_router = Router::new()
        .route("/", get(QuizHandler::get_all))
        .route(
            "/{id}",
            get(QuizHandler::get_one)
                .post(QuizHandler::create)
                .patch(QuizHandler::update)
                .delete(QuizHandler::delete),
        )
        .layer(middleware::from_fn(check_jwt))
        .with_state(state.clone());

    let share_quiz_router = Router::new()
        .route("/{id}/share", post(ShareQuizHandler::share))
        .route(
            "/{id}/shared_users",
            get(ShareQuizHandler::get_all_shared_users_of_quiz),
        )
        .route(
            "users/{id}/shared_quizzes",
            get(ShareQuizHandler::get_all_shared_quizzes_of_user),
        )
        .layer(middleware::from_fn(check_jwt))
        .with_state(state.clone());

    let quiz_question_router: Router = Router::new()
        .route(
            "/{id}/questions",
            post(QuizQuestionHandler::create).get(QuizQuestionHandler::get_all),
        )
        .route(
            "/{quiz_id}/questions/{quiz_question_id}",
            get(QuizQuestionHandler::get_by_id)
                .patch(QuizQuestionHandler::update)
                .delete(QuizQuestionHandler::delete),
        )
        .layer(middleware::from_fn(check_jwt))
        .with_state(state.clone());

    Router::new()
        .merge(quiz_router)
        .merge(share_quiz_router)
        .merge(quiz_question_router)
}
