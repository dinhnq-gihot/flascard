use {
    crate::{
        enums::{error::*, generic::PaginatedResponse},
        models::test::{
            CreateTest, CreateTestResponse, QueryTestParams, ResolveTestingQuestion, TestResponse,
            TestingQuestion,
        },
    },
    uuid::Uuid,
};

#[async_trait::async_trait]
pub trait TestService: Sync + Send {
    async fn get_all(
        &self,
        caller_id: Uuid,
        params: QueryTestParams,
    ) -> Result<PaginatedResponse<TestResponse>>;
    async fn get_by_id(&self, caller_id: Uuid, test_id: Uuid) -> Result<TestResponse>;
    async fn get_test_question(
        &self,
        caller_id: Uuid,
        test_id: Uuid,
        question_id: Uuid,
    ) -> Result<TestingQuestion>;
    async fn create(&self, caller_id: Uuid, payload: CreateTest) -> Result<CreateTestResponse>;
    async fn start(&self, caller_id: Uuid, test_id: Uuid) -> Result<TestingQuestion>;
    async fn resolve_test_question(
        &self,
        caller_id: Uuid,
        test_id: Uuid,
        question_id: Uuid,
        payload: ResolveTestingQuestion,
    ) -> Result<()>;
    async fn submit(&self, caller_id: Uuid, test_id: Uuid) -> Result<()>;
    async fn result(&self, caller_id: Uuid, test_id: Uuid) -> Result<()>;
    async fn review_solution(
        &self,
        caller_id: Uuid,
        test_id: Uuid,
        question_id: Uuid,
    ) -> Result<()>;
}
