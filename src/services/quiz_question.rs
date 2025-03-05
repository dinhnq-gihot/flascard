use {
    crate::{
        db::db_connection::Database,
        entities::{prelude::QuizQuestions, quiz_questions},
        enums::error::*,
    },
    std::sync::Arc,
    uuid::Uuid,
};

pub struct QuizQuestionService {
    db: Arc<Database>,
}

impl QuizQuestionService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn create_one(
        &self,
        id: Uuid,
    ) -> Result<(quiz_questions::Model, Vec<quiz_questions::Model>)> {
        let conn = self.db.get_connection().await;
        

        Ok(())
    }

    pub async fn update_one(&self, id: Uuid) -> Result<Option<quiz_questions::Model>> {
        // Ok(())
        !unimplemented!()
    }

    pub async fn get_by_id(&self, quiz_id: Uuid, id: Uuid) -> Result<quiz_questions::Model> {
        !unimplemented!()
    }

    pub async fn get_all(&self, quiz_id: Uuid) -> Result<quiz_questions::Model> {
        !unimplemented!()
    }
}
