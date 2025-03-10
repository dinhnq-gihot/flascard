use {
    crate::entities::sea_orm_active_enums::QuestionTypeEnum,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateQuizAnswer {
    pub content: String,
    pub is_answer: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuizQuestionRequest {
    pub question_content: String,
    pub answers: Vec<CreateQuizAnswer>,
    pub r#type: QuestionTypeEnum,
    pub question_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateQuizQuestionAnswer {
    pub id: Uuid,
    pub content: String,
    pub is_answer: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuizQuestionRequest {
    pub question_content: Option<String>,
    pub answers: Option<Vec<UpdateQuizQuestionAnswer>>,
}
