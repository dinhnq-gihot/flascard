use axum::{
    http::{StatusCode, Uri},
    response::IntoResponse,
};

pub mod user;

pub async fn fallback(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route: {uri}"))
}
