use {
    crate::{controllers::user_controller::*, middleware::jwt::check_jwt, server::AppState},
    axum::{middleware, routing::get, Router},
};

pub fn get_user_router(state: &AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(UserController::get_all_users)
                .delete(UserController::delete)
                .patch(UserController::update_self)
                .patch(UserController::update_password)
                .patch(UserController::update_role),
        )
        .layer(middleware::from_fn(check_jwt))
        .with_state(state.clone())
}
