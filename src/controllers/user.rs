use {
    crate::{
        debug,
        enums::{error::Result, generic::into_ok_response},
        models::user::{DeleteRequest, LoginRequest, RegisterUserRequest, UpdateUserRequest},
        r#static::BLACKLIST_TOKEN_VEC,
        server::AppState,
        utils::jwt::Claims,
    },
    axum::{extract::State, response::IntoResponse, Extension, Json},
    flashcard::only_role,
    std::sync::Arc,
};

pub struct UserHandler;

impl UserHandler {
    #[only_role("Staff")]
    pub async fn get_all_users(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
    ) -> Result<impl IntoResponse> {
        debug!("get_all_users: {caller:?}");

        let service = Arc::clone(&state.user_service);
        let users = service.get_all_users().await?;

        Ok(into_ok_response("success".into(), Some(users)))
    }

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

    pub async fn update(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Json(payload): Json<UpdateUserRequest>,
    ) -> Result<impl IntoResponse> {
        debug!("update request: {caller:?} {payload:?}");

        let service = Arc::clone(&state.user_service);
        let res = service.update(caller.id, payload).await?;

        Ok(into_ok_response("Updated successfully".into(), res))
    }

    #[only_role("Staff")]
    pub async fn delete(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Json(payload): Json<DeleteRequest>,
    ) -> Result<impl IntoResponse> {
        debug!("delete request: {payload:?}");

        let service = Arc::clone(&state.user_service);
        service.delete(payload.user_id).await?;

        Ok(into_ok_response(
            "Deleted successfully".into(),
            None::<String>,
        ))
    }

    pub async fn logout(Extension(token): Extension<String>) -> Result<impl IntoResponse> {
        debug!("logout: token: {token:?}");
        BLACKLIST_TOKEN_VEC.lock().push(token);

        Ok(into_ok_response(
            "Logout successfully".into(),
            None::<String>,
        ))
    }
}
