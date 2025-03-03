use {
    crate::{handlers::user::*, middleware::jwt::check_jwt, server::AppState},
    axum::{
        middleware,
        routing::{get, post},
        Router,
    },
};

pub fn get_user_router(state: &AppState) -> Router {
    let users_router = Router::new()
        .route(
            "/",
            get(UserHandler::get_all_users)
                .delete(UserHandler::delete)
                .patch(UserHandler::update),
        )
        .route("/logout", post(UserHandler::logout))
        .layer(middleware::from_fn(check_jwt))
        .with_state(state.clone());

    let auth_router = Router::new()
        .route("/register", post(UserHandler::register_user))
        .route("/login", post(UserHandler::login))
        .with_state(state.clone());

    Router::new().merge(users_router).merge(auth_router)
}
