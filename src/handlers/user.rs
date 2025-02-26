use {
    crate::server::AppState,
    anyhow::Result,
    axum::{extract::State, http::StatusCode, response::IntoResponse, Error, Json},
    std::sync::Arc,
};

#[axum::debug_handler]
pub async fn get_all_users(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let service = Arc::clone(&state.user_service);
    let users = service.get_all_users().await?;
    Ok((StatusCode::OK, Json(users)))
}
