use {
    crate::{
        debug,
        enums::{error::Result, generic::into_ok_response},
        models::user::{DeleteRequest, UpdateUserPassword, UpdateUserRequest, UpdateUserRole},
        server::AppState,
        utils::jwt::Claims,
    },
    axum::{extract::State, response::IntoResponse, Extension, Json},
    flashcard::only_role,
    std::sync::Arc,
};

pub struct UserController;

impl UserController {
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

    #[only_role("Staff", "User")]
    pub async fn update_self(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Json(payload): Json<UpdateUserRequest>,
    ) -> Result<impl IntoResponse> {
        debug!("update request: {caller:?} {payload:?}");

        let service = Arc::clone(&state.user_service);
        let res = service.update_self(caller.id, payload).await?;

        Ok(into_ok_response("Updated successfully".into(), res))
    }

    #[only_role("Staff", "User")]
    pub async fn update_password(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Json(payload): Json<UpdateUserPassword>,
    ) -> Result<impl IntoResponse> {
        debug!("update request: {caller:?} {payload:?}");

        let UpdateUserPassword {
            old_password,
            new_password,
        } = payload;

        let service = Arc::clone(&state.user_service);
        let res = service
            .update_password(caller.id, old_password, new_password)
            .await?;

        Ok(into_ok_response("Updated successfully".into(), res))
    }

    #[only_role("Staff")]
    pub async fn update_role(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Json(payload): Json<UpdateUserRole>,
    ) -> Result<impl IntoResponse> {
        debug!("update request: {caller:?} {payload:?}");

        let UpdateUserRole { user_id, new_role } = payload;

        let service = Arc::clone(&state.user_service);
        let res = service.update_role(caller.id, user_id, new_role).await?;

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
}
