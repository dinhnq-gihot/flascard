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
pub trait TestService {
    async fn get_all(
        &self,
        params: QueryTestParams,
    ) -> Result<PaginatedResponse<Vec<TestResponse>>>;
    async fn get_by_id(&self) -> Result<TestResponse>;
    async fn create(&self, payload: CreateTest) -> Result<CreateTestResponse>;
    async fn start(&self, test_id: Uuid) -> Result<TestingQuestion>;
    async fn get_test_question(&self, test_id: Uuid, question_id: Uuid) -> Result<TestingQuestion>;
    async fn resolve_test_question(
        &self,
        test_id: Uuid,
        question_id: Uuid,
        payload: ResolveTestingQuestion,
    ) -> Result<()>;
    async fn submit(&self, test_id: Uuid) -> Result<()>;
    async fn result(&self, test_id: Uuid) -> Result<()>;
    async fn review_solution(&self, test_id: Uuid, question_id: Uuid) -> Result<()>;
}
