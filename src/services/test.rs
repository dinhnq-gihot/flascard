use {crate::db::db::Database, std::sync::Arc, crate::entities::prelude::Tests};

pub struct TestService {
    db: Arc<Database>,
}

impl TestService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }


}
