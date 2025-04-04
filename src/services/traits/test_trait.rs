use {
    crate::{
        entities::{test_answers, test_question_results, tests},
        enums::{error::*, generic::PaginatedResponse},
        models::test::{
            CreateTest, CreateTestResponse, QueryTestParams, ResolveTestRequest, SaveTestAnswer,
            TestResponse, TestingQuestion,
        },
    },
    uuid::Uuid,
};

// Lấy tất cả test của user -> get_all
// Lấy test của user theo id -> get_by_id
// Tạo test -> create_one
// Bắt đầu test -> start_one
// Lấy test của user theo id -> get_test_question
// Lấy tất cả test result để hiện trạng thái các question đang làm ->
// get_all_test_results_of_test Đánh đáp test -> resolve_testing_question
// Nộp bài -> submit_one
// Xem kết quả -> get
//

#[async_trait::async_trait]
pub trait TestService: Sync + Send {
    // Test
    async fn create_one(&self, caller_id: Uuid, payload: CreateTest) -> Result<tests::Model>;
    async fn get_all(
        &self,
        caller_id: Uuid,
        params: QueryTestParams,
    ) -> Result<PaginatedResponse<tests::Model>>;
    async fn get_by_id(&self, caller_id: Uuid, test_id: Uuid) -> Result<tests::Model>;
    async fn start_one(&self, caller_id: Uuid, test_id: Uuid) -> Result<tests::Model>;

    // Test Answers
    async fn get_testing_question(
        &self,
        caller_id: Uuid,
        test_id: Uuid,
        quiz_question_id: Uuid,
    ) -> Result<TestingQuestion>;
    async fn get_all_testing_question_results(
        &self,
        test_id: Uuid,
    ) -> Result<Vec<test_question_results::Model>>;
    async fn resolve_testing_question(
        &self,
        caller_id: Uuid,
        test_id: Uuid,
        quiz_question_id: Uuid,
        payloads: ResolveTestRequest,
    ) -> Result<Option<tests::Model>>;
    async fn submit_one(
        &self,
        caller_id: Uuid,
        test_id: Uuid,
    ) -> Result<Vec<test_question_results::Model>>;
    async fn result(&self, caller_id: Uuid, test_id: Uuid) -> Result<()>;
    async fn review_solution(
        &self,
        caller_id: Uuid,
        test_id: Uuid,
        quiz_question_id: Uuid,
    ) -> Result<()>;
}
