use {
    crate::{
        enums::{error::*, generic::into_ok_response},
        server::AppState,
    },
    axum::{
        extract::{Path, State},
        response::IntoResponse,
        Json,
    },
    std::sync::Arc,
    uuid::Uuid,
};

pub struct ShareQuizHandler;

impl ShareQuizHandler {
    pub async fn share(
        State(state): State<AppState>,
        Path(quiz_id): Path<Uuid>,
        Json(payload): Json<Vec<Uuid>>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.shared_quiz_service);
        let res = service.create_share(quiz_id, payload).await?;

        Ok(into_ok_response("Shared successfully".into(), Some(res)))
    }

    pub async fn get_all_shared_quizzes_of_user(
        State(state): State<AppState>,
        Path(user_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.shared_quiz_service);
        let res = service.get_all_shared_quizzes_of_user(user_id).await?;

        Ok(into_ok_response("Success".into(), Some(res)))
    }

    pub async fn get_all_shared_users_of_quiz(
        State(state): State<AppState>,
        Path(quiz_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.shared_quiz_service);
        let res = service.get_all_shared_users_of_quiz(quiz_id).await?;

        Ok(into_ok_response("Success".into(), Some(res)))
    }
}
