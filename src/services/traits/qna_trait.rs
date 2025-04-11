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
pub trait QnAService: Sync + Send {
    // Create a question and add it to a set
    async fn create(&self, caller_id: Uuid, payload: CreateQnARequest) -> Result<questions::Model>;

    // Update information of a question and all answers of that question
    async fn update(
        &self,
        caller_id: Uuid,
        qna_id: Uuid,
        payload: UpdateQuestionRequest,
    ) -> Result<Option<questions::Model>>;

    // Delete a question
    async fn delete(&self, caller_id: Uuid, qna_id: Uuid) -> Result<()>;

    // Get a question with all answers
    async fn get_by_id(&self, caller_id: Uuid, qna_id: Uuid) -> Result<questions::Model>;

    // Get many
    async fn get_by_many_ids(
        &self,
        caller_id: Uuid,
        qna_ids: Vec<Uuid>,
    ) -> Result<Vec<questions::Model>>;

    // Get all questions according to the given params with pagination
    async fn get_all_of_set(
        &self,
        caller_id: Uuid,
        set_id: Uuid,
        params: QueryQuestionParams,
    ) -> Result<PaginatedResponse<questions::Model>>;
}
