use {
    crate::{
        db::db_connection::Database,
        entities::{prelude::Tests, test_states, tests},
        enums::{error::*, generic::PaginatedResponse},
    },
    std::sync::Arc,
    uuid::Uuid,
};

pub struct TestService {
    db: Arc<Database>,
}

impl TestService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn create_one(&self, quiz_id: Uuid, max_duration: u64) -> Result<()> {
        Ok(())
    }

    pub async fn get_all(&self) -> Result<PaginatedResponse<>> {
        Ok(())
    }

    pub async fn get_one(&self, test_id: Uuid) -> Result<(tests::Model, test_states::Model)> {
        // Ok(())
        !unimplemented!()
    }

    pub async fn start(&self, test_id: Uuid) -> Result<()> {
        Ok(())
    }

    pub async fn get_all_test_questions(&self, test_id: Uuid) -> Result<()> {
        Ok(())
    }

    pub async fn get_test_question(&self, test_id: Uuid, question_id: Uuid) -> Result<()> {
        Ok(())
    }

    // pub async fn answer_
}
