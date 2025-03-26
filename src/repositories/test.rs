use {
    crate::{
        db::db_connection::Database,
        entities::{test_results, test_states, tests},
        enums::{error::*, generic::PaginatedResponse},
        models::test::{QueryTestParams, ResolveTestingQuestion, UpdateTestParams},
    },
    std::sync::Arc,
    uuid::Uuid,
};

pub struct TestRepository {
    db: Arc<Database>,
}

impl TestRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn create_one(&self, quiz_id: Uuid, max_duration: u64) -> Result<tests::Model> {
        !unimplemented!()
    }

    pub async fn get_all(
        &self,
        caller_id: Uuid,
        params: QueryTestParams,
    ) -> Result<PaginatedResponse<(tests::Model, test_states::Model)>> {
        !unimplemented!()
    }

    pub async fn get_one(&self, test_id: Uuid) -> Result<(tests::Model, test_states::Model)> {
        // Ok(())
        !unimplemented!()
    }

    pub async fn update_one(
        &self,
        test_id: Uuid,
        params: UpdateTestParams,
    ) -> Result<(tests::Model, test_states::Model)> {
        !unimplemented!()
    }

    pub async fn get_test_question_result(
        &self,
        test_id: Uuid,
        quiz_question_id: Uuid,
    ) -> Result<Option<test_results::Model>> {
        !unimplemented!()
    }

    pub async fn create_test_question_result(
        &self,
        test_id: Uuid,
        quiz_question_id: Uuid,
        payload: ResolveTestingQuestion,
    ) -> Result<i32> {
        !unimplemented!()
    }
}
