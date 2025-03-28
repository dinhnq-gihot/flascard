use {
    crate::{controllers::set::SetController, middleware::jwt::check_jwt, server::AppState},
    axum::{
        middleware,
        routing::{get, post},
        Router,
    },
};

pub fn get_set_router(state: &AppState) -> Router {
    Router::new()
        .route("/", get(SetController::get_all).post(SetController::create))
        .route(
            "/{id}",
            get(SetController::get_by_id)
                .patch(SetController::update)
                .delete(SetController::delete),
        )
        .route("/{id}/share", post(SetController::share))
        .layer(middleware::from_fn(check_jwt))
        .with_state(state.clone())
}
