use {
    crate::{
        db::db_connection::Database,
        entities::{prelude::*, quizes, shared_quizes, users},
        enums::error::*,
    },
    sea_orm::{sea_query::OnConflict, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, Set},
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
        new_participants: Vec<Uuid>,
    ) -> Result<Vec<shared_quizes::Model>> {
        let conn = self.db.get_connection().await;

        let shared_quizes = SharedQuizes::find()
            .filter(shared_quizes::Column::QuizId.eq(quiz_id))
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)?;

        // Remove old participants not to be updated to share
        let users_to_unshare: Vec<Uuid> = shared_quizes
            .iter()
            .filter(|q| !new_participants.contains(&q.user_id))
            .map(|q| q.user_id)
            .collect();
        if !users_to_unshare.is_empty() {
            SharedQuizes::delete_many()
                .filter(shared_quizes::Column::QuizId.eq(quiz_id))
                .filter(shared_quizes::Column::UserId.is_in(users_to_unshare))
                .exec(&conn)
                .await
                .map_err(Error::DeleteFailed)?;
        }

        // Update new participants
        let new_sharing_quizes: Vec<shared_quizes::ActiveModel> = new_participants
            .into_iter()
            .map(|participant| {
                shared_quizes::ActiveModel {
                    user_id: Set(participant),
                    quiz_id: Set(quiz_id),
                    ..Default::default()
                }
            })
            .collect();
        let on_conflict = OnConflict::column(shared_quizes::Column::UserId)
            .do_nothing()
            .to_owned();

        SharedQuizes::insert_many(new_sharing_quizes)
            .on_conflict(on_conflict)
            .exec_with_returning_many(&conn)
            .await
            .map_err(Error::InsertFailed)
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
