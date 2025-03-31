use {
    crate::{
        enums::{error::*, generic::into_ok_response},
        models::quiz::{CreateQuizRequest, FilterQuizParams, UpdateQuizRequest},
        server::AppState,
        utils::jwt::Claims,
    },
    axum::{
        extract::{Path, Query, State},
        response::IntoResponse,
        Extension, Json,
    },
    std::sync::Arc,
    uuid::Uuid,
};

pub struct QuizController;

impl QuizController {
    pub async fn create(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Json(payload): Json<CreateQuizRequest>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_service);
        let res = service.create(caller.id, payload).await?;

        Ok(into_ok_response("created successfully".into(), Some(res)))
    }

    pub async fn update(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(quiz_id): Path<Uuid>,
        Json(payload): Json<UpdateQuizRequest>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_service);
        let res = service.update(caller.id, quiz_id, payload).await?;

        Ok(into_ok_response("Updated successfully".into(), res))
    }

    pub async fn delete(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(quiz_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_service);
        service.delete(quiz_id, caller.id).await?;

        Ok(into_ok_response(
            "Deleted successfully".into(),
            None::<String>,
        ))
    }

    pub async fn get_one(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_service);
        let quiz = service.get_by_id(caller.id, id).await?;

        Ok(into_ok_response("Success".into(), Some(quiz)))
    }

    pub async fn get_all(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Query(params): Query<FilterQuizParams>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_service);
        let quiz = service.get_all(caller.id, params).await?;

        Ok(into_ok_response("Success".into(), Some(quiz)))
    }

    pub async fn share(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(quiz_id): Path<Uuid>,
        Json(new_participant_ids): Json<Vec<Uuid>>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_service);
        let res = service
            .share(caller.id, quiz_id, new_participant_ids)
            .await?;

        Ok(into_ok_response("Shared successfully".into(), Some(res)))
    }

    pub async fn get_all_shared_users_of_quiz(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(quiz_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_service);
        let res = service
            .get_all_shared_users_of_quiz(caller.id, quiz_id)
            .await?;

        Ok(into_ok_response("Shared successfully".into(), Some(res)))
    }
}
