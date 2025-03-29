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

pub struct QnAController;

impl QnAController {
    pub async fn create(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Json(payload): Json<CreateQnARequest>,
    ) -> Result<impl IntoResponse> {
        debug!("create_question: caller: {caller:?}, payload: {payload:?}");

        let service = Arc::clone(&state.qna_service);
        let res = service.create(caller.id, payload).await?;

        Ok(into_ok_response("Created successfully".into(), Some(res)))
    }

    pub async fn update(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(qna_id): Path<Uuid>,
        Json(payload): Json<UpdateQuestionRequest>,
    ) -> Result<impl IntoResponse> {
        debug!("update_question: caller: {caller:?}, payload: {payload:?}");

        let service = Arc::clone(&state.qna_service);
        let res = service.update(caller.id, qna_id, payload).await?;

        Ok(into_ok_response("Updated successfully".into(), res))
    }

    pub async fn delete(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(qna_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.qna_service);
        service.delete(caller.id, qna_id).await?;

        Ok(into_ok_response(
            "Deleted successfully".into(),
            None::<String>,
        ))
    }

    pub async fn get_by_id(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(qna_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.qna_service);
        let res = service.get_by_id(caller.id, qna_id).await?;

        Ok(into_ok_response("success".into(), Some(res)))
    }

    pub async fn get_all(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Query(params): Query<QueryQuestionParams>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.qna_service);
        let res = service
            .get_all_of_set(caller.id, params.set_id, params)
            .await?;

        Ok(into_ok_response("success".into(), Some(res)))
    }
}
