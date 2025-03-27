use {
    crate::entities::{
        quiz_question_answers, quiz_questions, sea_orm_active_enums::QuestionTypeEnum,
    },
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuizQuestionAnswer {
    pub id: Option<Uuid>,
    pub content: String,
    pub is_answer: bool,
}

#[derive(Debug, Deserialize, Clone)]
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
    pub index: Option<i32>,
    pub point: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateManyQuizQuestionsRequest {
    pub question_id: Uuid,
    pub content: UpdateQuizQuestionRequest,
}

#[derive(Debug, Serialize)]
pub struct QuizQuestionResponse {
    pub question: quiz_questions::Model,
    pub answers: Vec<quiz_question_answers::Model>,
}
