use {
    crate::entities::{answers, sea_orm_active_enums::QuestionTypeEnum},
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Deserialize)]
pub struct CreateAnswer {
    pub content: String,
    pub is_correct: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateQnARequest {
    pub content: String,
    pub r#type: QuestionTypeEnum,
    pub answers: Vec<CreateAnswer>,
    pub set_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuestionRequest {
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAnswerRequest {
    pub content: Option<String>,
    pub is_correct: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct QueryQuestionParams {
    pub content: Option<String>,
    pub r#type: Option<String>,
    pub set_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct QnAResponse {
    pub id: Uuid,
    pub content: String,
    pub r#type: QuestionTypeEnum,
    pub answers: Vec<answers::Model>,
    pub set_id: Uuid,
}
