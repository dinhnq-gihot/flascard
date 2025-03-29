use {
    crate::entities::sea_orm_active_enums::QuestionTypeEnum,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct AnswerDTO {
    pub content: String,
    pub is_answer: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateQnARequest {
    pub content: String,
    pub r#type: QuestionTypeEnum,
    pub answers: Vec<AnswerDTO>,
    pub set_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuestionRequest {
    pub content: Option<String>,
    pub answers: Option<Vec<AnswerDTO>>,
}

#[derive(Debug, Deserialize)]
pub struct QueryQuestionParams {
    pub content: Option<String>,
    pub r#type: Option<String>,
    pub set_id: Uuid, // force
    pub creator_id: Option<Uuid>,
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
    pub answers: Vec<AnswerDTO>,
    pub set_id: Uuid,
}
