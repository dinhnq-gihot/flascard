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

pub struct QuizHandler;

impl QuizHandler {
    pub async fn create(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Json(payload): Json<CreateQuizRequest>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_service);
        let res = service.create_one(caller.id, payload).await?;

        Ok(into_ok_response("created successfully".into(), Some(res)))
    }

    pub async fn update(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(quiz_id): Path<Uuid>,
        Json(payload): Json<UpdateQuizRequest>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_service);
        let res = service.update_one(caller.id, quiz_id, payload).await?;

        Ok(into_ok_response("Updated successfully".into(), res))
    }

    pub async fn delete(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Path(quiz_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_service);
        service.delete_one(quiz_id, caller.id).await?;

        Ok(into_ok_response(
            "Deleted successfully".into(),
            None::<String>,
        ))
    }

    pub async fn get_one(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_service);
        let quiz = service.get_by_id(id).await?;

        Ok(into_ok_response("Success".into(), Some(quiz)))
    }

    pub async fn get_all(
        State(state): State<AppState>,
        Query(params): Query<FilterQuizParams>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_service);
        let quiz = service.get_all(params).await?;

        Ok(into_ok_response("Success".into(), Some(quiz)))
    }
}
