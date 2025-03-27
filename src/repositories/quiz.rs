use {
    crate::{
        db::db_connection::Database,
        entities::{
            prelude::{Quizes, *},
            quizes, shared_quizes, users,
        },
        enums::error::*,
        models::quiz::{CreateQuizRequest, FilterQuizParams, UpdateQuizRequest},
    },
    sea_orm::{
        sea_query::OnConflict, ActiveModelTrait, ColumnTrait, Condition, EntityTrait, JoinType,
        ModelTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set,
    },
    std::sync::Arc,
    uuid::Uuid,
};

pub struct QuizRepository {
    db: Arc<Database>,
}

impl QuizRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn create_one(
        &self,
        payload: CreateQuizRequest,
        creator_id: Uuid,
    ) -> Result<quizes::Model> {
        let conn = self.db.get_connection().await;
        let CreateQuizRequest {
            name,
            created_from,
            is_public,
            question_counts,
        } = payload;

        let question_counts =
            serde_json::to_value(question_counts).map_err(|e| Error::Anyhow(e.into()))?;

        quizes::ActiveModel {
            name: Set(name.unwrap_or("Untitled Quiz".to_string())),
            set_id: Set(created_from),
            creator_id: Set(creator_id),
            public_or_not: Set(is_public),
            question_counts: Set(question_counts),
            ..Default::default()
        }
        .insert(&conn)
        .await
        .map_err(Error::InsertFailed)
    }

    pub async fn update_one(
        &self,
        id: Uuid,
        payload: UpdateQuizRequest,
    ) -> Result<Option<quizes::Model>> {
        let conn = self.db.get_connection().await;
        let quiz = Quizes::find_by_id(id)
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?;

        let mut active_model: quizes::ActiveModel = quiz.clone().into();

        let mut updated = false;
        if let Some(p) = payload.is_public {
            active_model.is_published = Set(p);
            updated = true;
        }
        if let Some(publish) = payload.publish {
            active_model.is_published = Set(publish);
            updated = true;
        }
        // update new last question and set start question if is none
        if let Some(last_question_id) = payload.last_question_id {
            active_model.last_question = Set(Some(last_question_id));
            if quiz.start_question.is_none() {
                active_model.start_question = Set(Some(last_question_id));
            }
            updated = true;
        }

        if updated {
            Ok(Some(
                active_model
                    .update(&conn)
                    .await
                    .map_err(Error::UpdateFailed)?,
            ))
        } else {
            Ok(None)
        }
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

    pub async fn get_all(
        &self,
        caller_id: Uuid,
        params: FilterQuizParams,
    ) -> Result<Vec<quizes::Model>> {
        let conn = self.db.get_connection().await;
        let mut condition = Condition::all().add(quizes::Column::IsDeleted.eq(false));

        if let Some(set_id) = params.set_id {
            condition = condition.add(quizes::Column::SetId.eq(set_id));
        }
        // filter theo user
        // => nếu có creator_id trong query thì lấy creator_id theo creator_id hoặc
        // caller_id và user được share =>
        if let Some(creator_id) = params.creator_id {
            condition = condition.add(
                Condition::any()
                    .add(quizes::Column::CreatorId.eq(creator_id))
                    .add(quizes::Column::CreatorId.eq(caller_id))
                    .add(shared_quizes::Column::UserId.eq(caller_id)),
            );
        } else {
            condition = condition.add(
                Condition::any()
                    .add(quizes::Column::CreatorId.eq(caller_id))
                    .add(shared_quizes::Column::UserId.eq(caller_id)),
            );
        }
        let query = Quizes::find()
            .join(JoinType::InnerJoin, quizes::Relation::SharedQuizes.def())
            .filter(condition);

        let query = match &params.sort_direction {
            Some(direction) if direction == "asc" => query.order_by_asc(quizes::Column::CreatedAt),
            _ => query.order_by_desc(quizes::Column::CreatedAt),
        };

        query.all(&conn).await.map_err(Error::QueryFailed)
    }

    pub async fn get_all_public(&self, params: FilterQuizParams) -> Result<Vec<quizes::Model>> {
        let conn = self.db.get_connection().await;
        let mut condition = Condition::all()
            .add(quizes::Column::PublicOrNot.eq(true))
            .add(quizes::Column::IsDeleted.eq(false));

        if let Some(set_id) = params.set_id {
            condition = condition.add(quizes::Column::SetId.eq(set_id));
        }
        if let Some(creator_id) = params.creator_id {
            condition = condition.add(quizes::Column::CreatorId.eq(creator_id));
        }

        let query = Quizes::find().filter(condition);
        let query = match &params.sort_direction {
            Some(direction) if direction == "asc" => query.order_by_asc(quizes::Column::UpdatedAt),
            _ => query.order_by_desc(quizes::Column::UpdatedAt),
        };

        query.all(&conn).await.map_err(Error::QueryFailed)
    }

    pub async fn is_created_by(&self, quiz_id: Uuid, user_id: Uuid) -> Result<bool> {
        let conn = self.db.get_connection().await;
        let quiz = Quizes::find_by_id(quiz_id)
            .filter(quizes::Column::CreatorId.eq(user_id))
            .filter(quizes::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?;

        Ok(quiz.is_some())
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

    pub async fn is_shared_with(&self, quiz_id: Uuid, user_id: Uuid) -> Result<bool> {
        let conn = self.db.get_connection().await;
        let shared_quiz = SharedQuizes::find()
            .filter(shared_quizes::Column::QuizId.eq(quiz_id))
            .filter(shared_quizes::Column::UserId.eq(user_id))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?;

        Ok(shared_quiz.is_some())
    }
}
