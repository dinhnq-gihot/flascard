use {
    crate::{
        entities::{quiz_question_answers, quiz_questions},
        enums::error::*,
        models::quiz_question::{
            CreateQuizQuestionRequest, QuizQuestionResponse, UpdateQuizQuestionRequest,
        },
    },
    async_trait::async_trait,
    uuid::Uuid,
};

#[async_trait]
pub trait QuizQuestionService: Sync + Send {
    async fn create_one(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        payload: CreateQuizQuestionRequest,
    ) -> Result<(quiz_questions::Model, Vec<quiz_question_answers::Model>)>;

    async fn update_one(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        quiz_question_id: Uuid,
        payload: UpdateQuizQuestionRequest,
    ) -> Result<Option<quiz_questions::Model>>;

    async fn delete(&self, caller_id: Uuid, quiz_id: Uuid, quiz_question_id: Uuid) -> Result<()>;

    async fn get_by_id(
        &self,
        quiz_id: Uuid,
        quiz_question_id: Uuid,
    ) -> Result<QuizQuestionResponse>;

    async fn get_all(&self, quiz_id: Uuid) -> Result<Vec<quiz_questions::Model>>;
}
