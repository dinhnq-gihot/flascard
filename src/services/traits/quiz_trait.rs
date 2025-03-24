use {
    crate::{
        entities::quizes,
        enums::error::*,
        models::quiz::{CreateQuizRequest, FilterQuizParams, UpdateQuizRequest},
    },
    async_trait::async_trait,
    uuid::Uuid,
};

#[async_trait]
pub trait QuizService {
    async fn create_one(
        &self,
        payload: CreateQuizRequest,
        creator_id: Uuid,
    ) -> Result<quizes::Model>;
    async fn update_one(
        &self,
        quiz_id: Uuid,
        payload: UpdateQuizRequest,
    ) -> Result<Option<quizes::Model>>;
    async fn delete_one(&self, quiz_id: Uuid) -> Result<()>;
    async fn get_by_id(&self, quiz_id: Uuid) -> Result<quizes::Model>;
    async fn get_all(&self, params: FilterQuizParams) -> Result<Vec<quizes::Model>>;
}
