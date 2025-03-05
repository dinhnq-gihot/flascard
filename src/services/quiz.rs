use {
    crate::{
        db::db_connection::Database,
        entities::{
            prelude::{Quizes, SharedQuizes},
            quizes, shared_quizes,
        },
        enums::error::*,
        models::quiz::{CreateQuizRequest, FilterQuizParams, UpdateQuizRequest},
    },
    sea_orm::{
        sea_query::OnConflict, ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder,
        Set,
    },
    std::sync::Arc,
    uuid::Uuid,
};

pub struct QuizService {
    db: Arc<Database>,
}

impl QuizService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn create_one(
        &self,
        payload: CreateQuizRequest,
        creator_id: Uuid,
    ) -> Result<(quizes::Model, Vec<shared_quizes::Model>)> {
        let conn = self.db.get_connection().await;
        let CreateQuizRequest {
            created_from,
            is_public,
            question_counts,
            share_with,
        } = payload;

        let quiz = quizes::ActiveModel {
            set_id: Set(created_from),
            creator_id: Set(creator_id),
            public_or_not: Set(is_public),
            question_counts: Set(question_counts),
            ..Default::default()
        }
        .insert(&conn)
        .await
        .map_err(Error::InsertFailed)?;

        let sharing_quizes: Vec<shared_quizes::ActiveModel> = share_with
            .into_iter()
            .map(|participant_id| {
                shared_quizes::ActiveModel {
                    user_id: Set(creator_id),
                    quiz_id: Set(participant_id),
                    ..Default::default()
                }
            })
            .collect();
        let shared_quizes = SharedQuizes::insert_many(sharing_quizes)
            .exec_with_returning_many(&conn)
            .await
            .map_err(Error::InsertFailed)?;

        Ok((quiz, shared_quizes))
    }

    pub async fn update_one(
        &self,
        id: Uuid,
        payload: UpdateQuizRequest,
    ) -> Result<(Option<quizes::Model>, Option<Vec<shared_quizes::Model>>)> {
        let conn = self.db.get_connection().await;
        let mut quiz: quizes::ActiveModel = Quizes::find_by_id(id)
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?
            .into();

        let updated_quiz = if let Some(p) = payload.is_public {
            quiz.is_published = Set(p);
            Some(quiz.update(&conn).await.map_err(Error::UpdateFailed)?)
        } else {
            None
        };

        let new_shared_quizes = if let Some(new_participants) = payload.share_with {
            let shared_quizes = SharedQuizes::find()
                .filter(shared_quizes::Column::QuizId.eq(id))
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
                    .filter(shared_quizes::Column::QuizId.eq(id))
                    .filter(shared_quizes::Column::UserId.is_in(users_to_unshare))
                    .exec(&conn)
                    .await
                    .map_err(Error::DeleteFailed)?;
            }

            // Update new participants
            let new_sharing_quizes: Vec<shared_quizes::ActiveModel> = new_participants
                .into_iter()
                .map(|participant_id| {
                    shared_quizes::ActiveModel {
                        user_id: Set(participant_id),
                        quiz_id: Set(id),
                        ..Default::default()
                    }
                })
                .collect();
            let on_conflict = OnConflict::column(shared_quizes::Column::UserId)
                .do_nothing()
                .to_owned();
            Some(
                SharedQuizes::insert_many(new_sharing_quizes)
                    .on_conflict(on_conflict)
                    .exec_with_returning_many(&conn)
                    .await
                    .map_err(Error::InsertFailed)?,
            )
        } else {
            None
        };

        Ok((updated_quiz, new_shared_quizes))
    }

    pub async fn delete_one(&self, id: Uuid) -> Result<()> {
        let conn = self.db.get_connection().await;
        let mut quiz: quizes::ActiveModel = Quizes::find_by_id(id)
            .filter(quizes::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?
            .into();

        quiz.is_deleted = Set(true);
        quiz.update(&conn).await.map_err(Error::DeleteFailed)?;

        Ok(())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<quizes::Model> {
        let conn = self.db.get_connection().await;
        Quizes::find_by_id(id)
            .filter(quizes::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)
    }

    pub async fn get_all(&self, params: FilterQuizParams) -> Result<Vec<quizes::Model>> {
        let conn = self.db.get_connection().await;
        let mut query = Quizes::find().filter(quizes::Column::IsDeleted.eq(false));

        if let Some(set_id) = params.set_id {
            query = query.filter(quizes::Column::SetId.eq(set_id));
        }
        if let Some(creator_id) = params.creator_id {
            query = query.filter(quizes::Column::CreatorId.eq(creator_id));
        }

        let query = match &params.sort_direction {
            Some(direction) if direction == "asc" => query.order_by_asc(quizes::Column::CreatedAt),
            _ => query.order_by_desc(quizes::Column::CreatedAt),
        };

        query.all(&conn).await.map_err(Error::QueryFailed)
    }
}
