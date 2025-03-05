use {
    crate::entities::sea_orm_active_enums::PermissionEnum, serde::Deserialize, serde_json::Value,
    uuid::Uuid,
};

#[derive(Debug, Deserialize)]
pub struct CreateQuizRequest {
    pub created_from: Uuid,
    pub is_public: bool,
    pub question_counts: Value,
    pub share_with: Vec<Uuid>, // participants
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuizRequest {
    pub is_public: Option<bool>,
    pub share_with: Option<Vec<Uuid>>, // participants
}

#[derive(Debug, Deserialize)]
pub struct FilterQuizParams {
    pub set_id: Option<Uuid>,
    pub creator_id: Option<Uuid>,
    pub sort_direction: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ShareQuizForUser {
    pub user_id: Uuid,
    pub permission: PermissionEnum,
}
