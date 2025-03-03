use {
    crate::{
        enums::{error::*, generic::into_ok_response},
        models::set::{CreateSetRequest, ShareSetRequest, UpdateSetRequest},
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

        let CreateSetRequest {
            name,
            description,
            public_or_not,
        } = payload;

        let set = service
            .create_set(caller.id, name, description, public_or_not)
            .await?;

        Ok(into_ok_response("Created successfully".into(), Some(set)))
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
        let sets = service.get_all_set().await?;

        Ok(into_ok_response("success".into(), Some(sets)))
    }

    pub async fn update(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(id): Path<Uuid>,
        Json(payload): Json<UpdateSetRequest>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.set_service);

        service.is_owner(id, caller.id).await?;

        let UpdateSetRequest {
            name,
            description,
            public_or_not,
        } = payload;

        let set = service
            .update_set(id, name, description, public_or_not)
            .await?;

        Ok(into_ok_response("Updated successfully".into(), set))
    }

    pub async fn delete(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.set_service);

        service.is_owner(id, caller.id).await?;
        service.delete_set(id).await?;

        Ok(into_ok_response(
            "Deleted successfully".into(),
            None::<String>,
        ))
    }

    pub async fn get_all_sets_of_user(
        State(state): State<AppState>,
        Path(user_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let shared_set_service = Arc::clone(&state.shared_set_service);
        let set_service = Arc::clone(&state.set_service);

        let owned_sets = set_service.get_by_owner_id(user_id).await?;
        let shared_sets = shared_set_service
            .get_all_shared_sets_of_user(user_id)
            .await?;

        let mut all_sets = owned_sets;
        all_sets.extend(shared_sets);

        Ok(into_ok_response("Success".into(), Some(all_sets)))
    }

    pub async fn share(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Json(payload): Json<ShareSetRequest>,
    ) -> Result<impl IntoResponse> {
        let shared_set_service = Arc::clone(&state.shared_set_service);
        let set_service = Arc::clone(&state.set_service);

        let ShareSetRequest {
            user_id,
            set_id,
            permission,
        } = payload;

        if shared_set_service
            .get_shared_set(set_id, user_id)
            .await?
            .is_none()
            && set_service.is_owner(set_id, caller.id).await.is_ok()
        {
            let res = shared_set_service
                .create_shared_set(user_id, set_id, permission)
                .await?;
            return Ok(into_ok_response("Shared successfully".into(), Some(res)));
        }

        Err(Error::PermissionDenied)
    }
}
