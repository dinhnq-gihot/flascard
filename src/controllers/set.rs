use {
    crate::{
        enums::{error::*, generic::into_ok_response},
        models::set::{CreateSetRequest, ShareSetForUser, UpdateSetRequest},
        server::AppState,
        utils::jwt::Claims,
    },
    axum::{
        extract::{Path, State},
        response::IntoResponse,
        Extension, Json,
    },
    std::sync::Arc,
    uuid::Uuid,
};

pub struct SetHandler;

impl SetHandler {
    pub async fn create(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Json(payload): Json<CreateSetRequest>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.set_service);
        let res = service.create(caller.id, payload).await?;

        Ok(into_ok_response("Created successfully".into(), Some(res)))
    }

    pub async fn get_by_id(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.set_service);
        let set = service.get_by_id(id).await?;

        Ok(into_ok_response("Success".into(), Some(set)))
    }

    pub async fn get_all(State(state): State<AppState>) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.set_service);
        let sets = service.get_all().await?;

        Ok(into_ok_response("success".into(), Some(sets)))
    }

    pub async fn update(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(set_id): Path<Uuid>,
        Json(payload): Json<UpdateSetRequest>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.set_service);
        let res = service.update(caller.id, set_id, payload).await?;

        Ok(into_ok_response("Updated successfully".into(), res))
    }

    pub async fn delete(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(set_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.set_service);
        service.delete(caller.id, set_id).await?;

        Ok(into_ok_response(
            "Deleted successfully".into(),
            None::<String>,
        ))
    }

    pub async fn get_all_sets_of_user(
        State(state): State<AppState>,
        Path(user_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.set_service);
        let res = service.get_all_sets_of_user(user_id).await?;

        Ok(into_ok_response("Success".into(), Some(res)))
    }

    pub async fn share(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(set_id): Path<Uuid>,
        Json(payload): Json<Vec<ShareSetForUser>>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.set_service);
        let res = service.share(caller.id, set_id, payload).await?;

        Ok(into_ok_response("Shared successfully".into(), Some(res)))
    }
}
