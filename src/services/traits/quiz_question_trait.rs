use {
    crate::{
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
    async fn create(
        &self,
        caller_id: Uuid,
        quiz_question_id: Uuid,
        payloads: Vec<CreateQuizQuestionRequest>,
    ) -> Result<Vec<QuizQuestionResponse>>;

    async fn update(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        payloads: Vec<UpdateQuizQuestionRequest>,
    ) -> Result<Vec<QuizQuestionResponse>>;

    async fn delete(&self, caller_id: Uuid, quiz_id: Uuid, quiz_question_id: Uuid) -> Result<()>;

    async fn get_by_id(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        quiz_question_id: Uuid,
    ) -> Result<QuizQuestionResponse>;

    async fn get_by_index(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        quiz_question_index: i32,
    ) -> Result<QuizQuestionResponse>;

    async fn get_all(&self, caller_id: Uuid, quiz_id: Uuid) -> Result<Vec<QuizQuestionResponse>>;
}
