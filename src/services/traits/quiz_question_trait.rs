use {
    crate::{
        entities::quiz_questions,
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
    ) -> Result<QuizQuestionResponse>;

    async fn update_one(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        quiz_question_id: Uuid,
        payload: UpdateQuizQuestionRequest,
    ) -> Result<Option<QuizQuestionResponse>>;

    async fn update_many() {
        
    }

    async fn delete(&self, caller_id: Uuid, quiz_id: Uuid, quiz_question_id: Uuid) -> Result<()>;

    async fn get_by_id(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        quiz_question_id: Uuid,
    ) -> Result<QuizQuestionResponse>;

    async fn get_all(&self, caller_id: Uuid, quiz_id: Uuid) -> Result<Vec<quiz_questions::Model>>;
}
