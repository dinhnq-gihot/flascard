use axum::{
    http::{StatusCode, Uri},
    response::IntoResponse,
};

pub mod auth_controller;
pub mod qna_controller;
pub mod quiz_controller;
pub mod quiz_question_controller;
pub mod set_controller;
pub mod test_controller;
pub mod user_controller;

pub async fn fallback(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route: {uri}"))
}
