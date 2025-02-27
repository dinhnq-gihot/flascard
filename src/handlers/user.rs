use {
    crate::{
        debug,
        enums::{error::Result, generic::into_ok_response},
        models::user::{RegisterUserRequest, UserResponse},
        server::AppState,
    },
    axum::{extract::State, response::IntoResponse, Json},
    std::sync::Arc,
};

pub struct UserHandler;

impl UserHandler {
    pub async fn get_all_users(State(state): State<AppState>) -> Result<impl IntoResponse> {
        debug!("get_all_users");

        let service = Arc::clone(&state.user_service);
        let users = service.get_all_users().await?;

        Ok(into_ok_response("success".into(), Some(users)))
    }

    // #[axum::debug_handler]
    pub async fn register_user(
        State(state): State<AppState>,
        Json(payload): Json<RegisterUserRequest>,
    ) -> Result<impl IntoResponse> {
        debug!("register_user: {payload:?}");
        let service = Arc::clone(&state.user_service);

        let RegisterUserRequest {
            email,
            password,
            name,
            role,
        } = payload;

        let user: UserResponse = service
            .create_user(email, password, name, role)
            .await?
            .into();

        Ok(into_ok_response(
            "registered successfully".into(),
            Some(user),
        ))
    }
}
