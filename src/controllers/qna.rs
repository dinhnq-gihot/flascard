use {
    crate::{
        debug,
        enums::{error::*, generic::into_ok_response},
        models::qna::*,
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

pub struct QnAHandler;

impl QnAHandler {
    pub async fn create_question(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Json(payload): Json<CreateQnARequest>,
    ) -> Result<impl IntoResponse> {
        debug!("create_question: caller: {caller:?}, payload: {payload:?}");

        let service = Arc::clone(&state.qna_service);

        let (question, answers) = service.create(payload, caller.id).await?;
        let res = QnAResponse {
            id: question.id,
            content: question.content,
            r#type: question.r#type,
            answers,
            set_id: question.set_id,
        };

        Ok(into_ok_response("Created successfully".into(), Some(res)))
    }

    pub async fn update_question(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(id): Path<Uuid>,
        Json(payload): Json<UpdateQuestionRequest>,
    ) -> Result<impl IntoResponse> {
        debug!("update_question: caller: {caller:?}, payload: {payload:?}");

        let service = Arc::clone(&state.qna_service);

        let res = if service.is_creator_of_question(id, caller.id).await? {
            service.update_question(id, payload.content).await?
        } else {
            return Err(Error::PermissionDenied);
        };

        Ok(into_ok_response("Updated successfully".into(), res))
    }

    pub async fn update_answer(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(id): Path<Uuid>,
        Json(payload): Json<UpdateAnswerRequest>,
    ) -> Result<impl IntoResponse> {
        debug!("update_answer: caller: {caller:?}, payload: {payload:?}");

        let service = Arc::clone(&state.qna_service);

        let res = if service.is_creator_of_answer(id, caller.id).await? {
            service.update_answer(id, payload).await?
        } else {
            return Err(Error::PermissionDenied);
        };

        Ok(into_ok_response("Updated successfully".into(), res))
    }

    pub async fn delete_question(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.qna_service);

        if service.is_creator_of_question(id, caller.id).await? {
            service.delete_question(id).await?;
        } else {
            return Err(Error::PermissionDenied);
        }

        Ok(into_ok_response(
            "Deleted successfully".into(),
            None::<String>,
        ))
    }

    pub async fn delete_answer(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.qna_service);

        if service.is_creator_of_answer(id, caller.id).await? {
            service.delete_answer(id).await?;
        } else {
            return Err(Error::PermissionDenied);
        }

        Ok(into_ok_response(
            "Deleted successfully".into(),
            None::<String>,
        ))
    }

    pub async fn get_by_id(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.qna_service);

        let (question, answers) = service.get_by_id(id).await?;

        let res = QnAResponse {
            id: question.id,
            content: question.content,
            r#type: question.r#type,
            answers,
            set_id: question.set_id,
        };

        Ok(into_ok_response("success".into(), Some(res)))
    }

    pub async fn get_all(
        State(state): State<AppState>,
        Query(params): Query<QueryQuestionParams>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.qna_service);

        let res = service.get_all(params).await?;
        Ok(into_ok_response("success".into(), Some(res)))
    }
}
