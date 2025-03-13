use {
    crate::{
        db::db_connection::Database,
        entities::{prelude::Quizes, quizes},
        enums::error::*,
        models::quiz::{CreateQuizRequest, FilterQuizParams, UpdateQuizRequest},
    },
    sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set},
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

    pub async fn is_created_by(&self, quiz_id: Uuid, user_id: Uuid) -> Result<bool> {
        let conn = self.db.get_connection().await;
        let quiz = Quizes::find_by_id(quiz_id)
            .filter(quizes::Column::CreatorId.eq(user_id))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?;

        Ok(quiz.is_some())
    }
}
