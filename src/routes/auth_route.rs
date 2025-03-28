use {
    crate::{
        controllers::auth_controller::AuthController, middleware::jwt::check_jwt, server::AppState,
    },
    axum::{middleware, routing::post, Router},
};

pub fn get_auth_router(state: &AppState) -> Router {
    let logout_router = Router::new()
        .route("/logout", post(AuthController::logout))
        .layer(middleware::from_fn(check_jwt));

    let auth_router = Router::new()
        .route("/register", post(AuthController::register_user))
        .route("/login", post(AuthController::login))
        .with_state(state.clone());

    Router::new().merge(logout_router).merge(auth_router)
}
