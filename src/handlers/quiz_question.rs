use {
    crate::{
        enums::{error::*, generic::into_ok_response},
        models::quiz_question::{CreateQuizQuestionRequest, UpdateQuizQuestionRequest},
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
        Path(quiz_id): Path<Uuid>,
        Json(payload): Json<CreateQuizQuestionRequest>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_question_service);

        if !validate_answer(&payload.r#type, &payload.answers) {
            return Err(Error::InvalidAnswer);
        }
        let res = service.create_one(quiz_id, payload).await?;
        Ok(into_ok_response("Created successfully".into(), Some(res)))
    }

    pub async fn update(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(quiz_id): Path<Uuid>,
        Path(quiz_question_id): Path<Uuid>,
        Json(payload): Json<UpdateQuizQuestionRequest>,
    ) -> Result<impl IntoResponse> {
        let quiz_question_service = Arc::clone(&state.quiz_question_service);
        let quiz_service = Arc::clone(&state.quiz_service);

        let quiz = quiz_service.get_by_id(quiz_id).await?;
        let quiz_question = quiz_question_service
            .get_by_id(quiz_question_id, quiz_id)
            .await?;

        if quiz.creator_id != caller.id {
            return Err(Error::PermissionDenied);
        }
        if quiz.is_published {
            return Err(Error::PermissionDenied);
        }
        if let Some(answers) = &payload.answers {
            if !validate_answer(&quiz_question.r#type, answers)
                || !all_quiz_answers_contain_id(answers)
            {
                return Err(Error::InvalidAnswer);
            }
        }

        let res = quiz_question_service
            .update_one(quiz_question_id, quiz_id, payload)
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
        let quiz_service = Arc::clone(&state.quiz_service);

        let quiz = quiz_service.get_by_id(quiz_id).await?;
        if quiz.creator_id != caller.id {
            return Err(Error::PermissionDenied);
        }

        service.delete(quiz_question_id, quiz_id).await?;

        Ok(())
    }

    pub async fn get_by_id(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(quiz_id): Path<Uuid>,
        Path(quiz_question_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_question_service);
        let quiz_service = Arc::clone(&state.quiz_service);
        let share_quiz_service = Arc::clone(&state.shared_quiz_service);

        if !(quiz_service.is_created_by(quiz_id, caller.id).await?
            || share_quiz_service.is_shared(quiz_id, caller.id).await?)
        {
            return Err(Error::PermissionDenied);
        }
        let res = service.get_by_id(quiz_question_id, quiz_id).await?;

        Ok(into_ok_response("success".into(), Some(res)))
    }

    pub async fn get_all(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
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
