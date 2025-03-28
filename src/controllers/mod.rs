use axum::{
    http::{StatusCode, Uri},
    response::IntoResponse,
};

pub mod qna;
pub mod quiz;
pub mod set;
pub mod share_quiz;
pub mod user_controller;
pub mod quiz_question;
pub mod test;
pub mod auth_controller;

pub async fn fallback(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route: {uri}"))
}
