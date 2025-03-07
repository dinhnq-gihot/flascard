use {
    crate::{
        db::db_connection::Database,
        entities::{prelude::Tests, tests},
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

    pub async fn create(&self, quiz_id: Uuid) {}

    pub async fn get_one() {
        
    }
}
