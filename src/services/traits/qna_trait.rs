use {
    crate::{
        entities::questions,
        enums::{error::*, generic::PaginatedResponse},
        models::qna::{CreateQnARequest, QueryQuestionParams, UpdateQuestionRequest},
    },
    async_trait::async_trait,
    uuid::Uuid,
};

#[async_trait]
pub trait QnAService {
    async fn create(&self, caller_id: Uuid, payload: CreateQnARequest) -> Result<questions::Model>;
    async fn update(
        &self,
        caller_id: Uuid,
        qna_id: Uuid,
        payload: UpdateQuestionRequest,
    ) -> Result<Option<questions::Model>>;
    async fn delete(&self, caller_id: Uuid, qna_id: Uuid) -> Result<()>;
    async fn get_by_id(&self, qna_id: Uuid) -> Result<questions::Model>;
    async fn get_all(
        &self,
        params: QueryQuestionParams,
    ) -> Result<PaginatedResponse<questions::Model>>;
}
