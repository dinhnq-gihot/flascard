use {
    crate::entities::sea_orm_active_enums::QuestionTypeEnum,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct QuizAnswer {
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuizQuestionRequest {
    pub question_content: String,
    pub answers: Vec<QuizAnswer>,
    pub r#type: QuestionTypeEnum,
    pub question_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuizQuestionRequest {
    pub question_content: Option<String>,
    pub answers: Option<Vec<QuizAnswer>>,
}
