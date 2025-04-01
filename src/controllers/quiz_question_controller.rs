use {
    crate::{
        enums::{error::*, generic::into_ok_response},
        models::quiz_question::{CreateQuizQuestionRequest, UpdateQuizQuestionRequest},
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

pub struct QuizQuestionController;

impl QuizQuestionController {
    pub async fn create(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(quiz_id): Path<Uuid>,
        Json(payloads): Json<Vec<CreateQuizQuestionRequest>>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_question_service);
        let res = service.create(caller.id, quiz_id, payloads).await?;

        Ok(into_ok_response("Created successfully".into(), Some(res)))
    }

    pub async fn update(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(quiz_id): Path<Uuid>,
        Json(payloads): Json<Vec<UpdateQuizQuestionRequest>>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_question_service);
        let res = service.update(caller.id, quiz_id, payloads).await?;

        Ok(into_ok_response("Updated successfully".into(), Some(res)))
    }

    pub async fn delete(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(quiz_id): Path<Uuid>,
        Path(quiz_question_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_question_service);
        service.delete(caller.id, quiz_id, quiz_question_id).await?;

        Ok(())
    }

    pub async fn get_by_id(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(quiz_id): Path<Uuid>,
        Path(quiz_question_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_question_service);
        let res = service
            .get_by_id(caller.id, quiz_question_id, quiz_id)
            .await?;

        Ok(into_ok_response("success".into(), Some(res)))
    }

    pub async fn get_all(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(quiz_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_question_service);
        let res = service.get_all(caller.id, quiz_id).await?;

        Ok(into_ok_response("success".into(), Some(res)))
    }
}
