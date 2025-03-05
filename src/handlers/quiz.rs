use {
    crate::{
        enums::error::*,
        models::quiz::{CreateQuizRequest, UpdateQuizRequest},
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

pub struct QuizHandler;

impl QuizHandler {
    pub async fn create(
        State(state): State<AppState>,
        Extension(caller): Extension<Claims>,
        Json(payload): Json<CreateQuizRequest>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.quiz_service);

        let res = service.create_one(payload, caller.id).await?;

        Ok(())
    }

    pub async fn update(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
        Json(payload): Json<UpdateQuizRequest>,
    ) -> Result<impl IntoResponse> {
        Ok(())
    }

    pub async fn delete(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        Ok(())
    }

    pub async fn get_one(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        Ok(())
    }

    pub async fn get_all(State(state): State<AppState>) -> Result<impl IntoResponse> {
        Ok(())
    }
}
