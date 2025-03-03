use {
    crate::{handlers::set::*, middleware::jwt::check_jwt, server::AppState},
    axum::{
        middleware,
        routing::{get, post},
        Router,
    },
};

pub fn get_set_router(state: &AppState) -> Router {
    Router::new()
        .route("/", get(SetHandler::get_all).post(SetHandler::create))
        .route(
            "/{id}",
            get(SetHandler::get_by_id)
                .patch(SetHandler::update)
                .delete(SetHandler::delete),
        )
        .route("/users/{id}/sets/", get(SetHandler::get_all_sets_of_user))
        .route("/share", post(SetHandler::share))
        .layer(middleware::from_fn(check_jwt))
        .with_state(state.clone())
}
