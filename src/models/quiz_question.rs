use {
    crate::entities::sea_orm_active_enums::QuestionTypeEnum,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct QuizQuestionAnswer {
    pub id: Option<Uuid>,
    pub content: String,
    pub is_answer: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuizQuestionRequest {
    pub question_content: String,
    pub answers: Vec<QuizQuestionAnswer>,
    pub r#type: QuestionTypeEnum,
    pub question_id: Option<Uuid>,
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct UpdateQuizQuestionAnswer {
//     pub content: String,
//     pub is_answer: bool,
// }

#[derive(Debug, Deserialize)]
pub struct UpdateQuizQuestionRequest {
    pub question_content: Option<String>,
    pub answers: Option<Vec<QuizQuestionAnswer>>,
    pub previous_question_id: Option<Uuid>,
    pub next_question_id: Option<Uuid>,
}
