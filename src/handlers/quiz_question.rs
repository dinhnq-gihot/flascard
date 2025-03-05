use {
    crate::{enums::error::*, models::quiz::ShareQuizForUser, server::AppState},
    axum::{
        extract::{Path, State},
        response::IntoResponse,
        Json,
    },
    uuid::Uuid,
};

pub struct QuizQuestionHandler;

impl QuizQuestionHandler {
    pub async fn share(
        State(state): State<AppState>,
        Path(quiz_id): Path<Uuid>,
        Json(payload): Json<Vec<ShareQuizForUser>>,
    ) -> Result<impl IntoResponse> {
        Ok(())
    }

    pub async fn get_all_shared_quizzes_of_user(
        State(state): State<AppState>,
        Path(user_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        Ok(())
    }

    pub async fn get_all_shared_users_of_quiz(
        State(state): State<AppState>,
        Path(quiz_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        Ok(())
    }
}
