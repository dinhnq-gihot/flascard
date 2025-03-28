use {
    crate::{
        debug,
        enums::{error::*, generic::into_ok_response},
        models::user::{LoginRequest, RegisterUserRequest},
        r#static::BLACKLIST_TOKEN_VEC,
        server::AppState,
        utils::jwt::Claims,
    },
    axum::{extract::State, response::IntoResponse, Extension, Json},
    flashcard::only_role,
    std::sync::Arc,
};

pub struct AuthController;

impl AuthController {
    pub async fn register_user(
        State(state): State<AppState>,
        Json(payload): Json<RegisterUserRequest>,
    ) -> Result<impl IntoResponse> {
        debug!("register_user: {payload:?}");

        let service = Arc::clone(&state.user_service);
        let user = service.register_user(payload).await?;

        Ok(into_ok_response(
            "registered successfully".into(),
            Some(user),
        ))
    }

    pub async fn login(
        State(state): State<AppState>,
        Json(payload): Json<LoginRequest>,
    ) -> Result<impl IntoResponse> {
        debug!("login request: {payload:?}");

        let service = Arc::clone(&state.user_service);
        let res = service.login(payload).await?;

        Ok(into_ok_response("login successfully".into(), Some(res)))
    }

    #[only_role("Staff", "User")]
    pub async fn logout(
        Extension(caller): Extension<Claims>,
        Extension(token): Extension<String>,
    ) -> Result<impl IntoResponse> {
        debug!("logout: token: {token:?}");
        BLACKLIST_TOKEN_VEC.lock().push(token);

        Ok(into_ok_response(
            "Logout successfully".into(),
            None::<String>,
        ))
    }
}
