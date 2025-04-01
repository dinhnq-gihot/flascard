use {
    crate::entities::{
        quiz_question_answers, quiz_questions, sea_orm_active_enums::QuestionTypeEnum,
    },
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateQuizQuestionAnswer {
    pub content: String,
    pub is_answer: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateQuizQuestionRequest {
    pub question_content: String,
    pub answers: Vec<CreateQuizQuestionAnswer>,
    pub r#type: QuestionTypeEnum,
    pub sample_id: Option<Uuid>,
    pub point: i32,
    pub index: i32,
    pub explaination: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateQuizQuestionAnswer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_answer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiz_question_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateQuizQuestionContent {
    pub question_content: Option<String>,
    #[serde(skip_serializing)]
    pub answers: Option<Vec<UpdateQuizQuestionAnswer>>,
    pub r#type: Option<QuestionTypeEnum>,
    pub point: Option<i32>,
    pub index: Option<i32>,
    pub explaination: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuizQuestionRequest {
    pub question_id: Uuid,
    pub content: UpdateQuizQuestionContent,
}

#[derive(Debug, Serialize)]
pub struct QuizQuestionResponse {
    pub question: quiz_questions::Model,
    pub answers: Vec<quiz_question_answers::Model>,
}

#[derive(Debug, Serialize)]
pub struct MutationQuizQuestionRequest<T> {
    pub quiz_question_id: Uuid,
    pub payload: T,
}
