use {
    crate::{db::db::Database, services::user::UserService},
    anyhow::Result,
    axum::{http::StatusCode, response::IntoResponse, Json},
    std::sync::Arc,
};

pub struct UserHandler {
    service: UserService,
}

impl UserHandler {
    pub fn new(db: Arc<Database>) -> Self {
        let service = UserService::new(db);
        Self { service }
    }

    pub async fn get_all_users(&self) -> Result<impl IntoResponse> {
        let users = self.service.get_all_users().await?;
        Ok((StatusCode::OK, Json(users)))
    }
}
