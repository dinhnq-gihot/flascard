use {
    crate::{
        enums::{error::*, generic::into_ok_response},
        models::{
            quiz::UpdateQuizRequest,
            quiz_question::{CreateQuizQuestionRequest, UpdateQuizQuestionRequest},
        },
        server::AppState,
        utils::{
            jwt::Claims,
            validator::{all_quiz_answers_contain_id, validate_answer},
        },
    },
    axum::{
        extract::{Path, State},
        response::IntoResponse,
        Extension, Json,
    },
    std::sync::Arc,
    uuid::Uuid,
};

pub struct QuizQuestionHandler;

impl QuizQuestionHandler {
    pub async fn create(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(quiz_id): Path<Uuid>,
        Json(payload): Json<CreateQuizQuestionRequest>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_question_service);
        let res = service.create_one(caller.id, quiz_id, payload).await?;

        Ok(into_ok_response("Created successfully".into(), Some(res)))
    }

    pub async fn update(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(quiz_id): Path<Uuid>,
        Path(quiz_question_id): Path<Uuid>,
        Json(payload): Json<UpdateQuizQuestionRequest>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_question_service);
        let res = service
            .update_one(caller.id, quiz_id, quiz_question_id, payload)
            .await?;

        Ok(into_ok_response("Updated successfully".into(), res))
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
        let res = service.get_by_id(caller.id,  quiz_question_id, quiz_id).await?;

        Ok(into_ok_response("success".into(), Some(res)))
    }

    pub async fn get_all(
        State(state): State<AppState>,
        // Extension(caller): Extension<Claims>,
        Path(quiz_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_question_service);
        let quiz_service = Arc::clone(&state.quiz_service);
        let share_quiz_service = Arc::clone(&state.shared_quiz_service);

        if !(quiz_service.is_created_by(quiz_id, caller.id).await?
            || share_quiz_service.is_shared(quiz_id, caller.id).await?)
        {
            return Err(Error::PermissionDenied);
        }
        let res = service.get_all(quiz_id).await?;

        Ok(into_ok_response("success".into(), Some(res)))
    }
}
