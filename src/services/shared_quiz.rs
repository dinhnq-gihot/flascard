use {
    crate::{
        db::db_connection::Database,
        entities::{prelude::*, quizes, shared_quizes, users},
        enums::error::*,
        models::quiz::ShareQuizForUser,
    },
    sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter},
    std::sync::Arc,
    uuid::Uuid,
};
pub struct SharedQuizService {
    db: Arc<Database>,
}

impl SharedQuizService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn create_share(
        &self,
        quiz_id: Uuid,
        users: Vec<ShareQuizForUser>,
    ) -> Result<Vec<shared_quizes::Model>> {
        Ok(())
    }

    pub async fn get_all_shared_quizzes_of_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<quizes::Model>> {
        let conn = self.db.get_connection().await;
        let user = Users::find_by_id(user_id)
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?;

        user.find_related(Quizes)
            .filter(quizes::Column::IsDeleted.eq(false))
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)
    }

    pub async fn get_all_shared_users_of_quiz(&self, quiz_id: Uuid) -> Result<Vec<users::Model>> {
        let conn = self.db.get_connection().await;
        let quiz = Quizes::find_by_id(quiz_id)
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?;

        quiz.find_related(Users)
            .filter(users::Column::IsDeleted.eq(false))
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)
    }
}
