use {
    crate::{
        db::db_connection::Database,
        entities::{
            prelude::{Quizes, *},
            quizes, shared_quizes, users,
        },
        enums::{error::*, generic::PaginatedResponse},
        models::{
            quiz::{
                CreateQuizRequest, FilterQuizParams, QuestionCounts, QuizWithVisibility,
                UpdateQuizRequest,
            },
            user::UserModel,
        },
    },
    chrono::Utc,
    sea_orm::{
        sea_query::OnConflict, ActiveModelTrait, ColumnTrait, Condition, EntityTrait, JoinType,
        ModelTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set,
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
        let CreateQuizRequest { name, is_public } = payload;

        let question_counts =
            serde_json::to_value(QuestionCounts::default()).map_err(|e| Error::Anyhow(e.into()))?;

        quizes::ActiveModel {
            name: Set(name.unwrap_or("Untitled Quiz".to_string())),
            creator_id: Set(creator_id),
            is_public: Set(is_public),
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
        let mut active_model: quizes::ActiveModel = Quizes::find_by_id(id)
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?
            .into();

        let mut updated = false;

        if let Some(name) = payload.name {
            active_model.name = Set(name);
            updated = true;
        }
        if let Some(p) = payload.is_public {
            active_model.is_published = Set(p);
            updated = true;
        }
        if let Some(publish) = payload.is_publish {
            active_model.is_published = Set(publish);
            active_model.publish_at = Set(Some(Utc::now().naive_utc()));

            updated = true;
        }
        if let Some(counts) = payload.question_counts {
            active_model.question_counts =
                Set(serde_json::to_value(counts).map_err(|e| Error::Anyhow(e.into()))?);
            updated = true;
        }
        if let Some(total_point) = payload.total_point {
            active_model.total_point = Set(total_point);
            updated = true;
        }

        if updated {
            active_model.publish_at = Set(Some(Utc::now().naive_utc()));

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

    pub async fn get_by_id(&self, caller_id: Uuid, id: Uuid) -> Result<quizes::Model> {
        let conn = self.db.get_connection().await;

        // WHERE (creator_id = caller_id OR shared_quizes.user_id = caller_id OR
        // is_public = true) AND is_delete = false
        let condition = Condition::all()
            .add(quizes::Column::IsDeleted.eq(false))
            .add(
                Condition::any()
                    .add(quizes::Column::CreatorId.eq(caller_id))
                    .add(shared_quizes::Column::UserId.eq(caller_id))
                    .add(quizes::Column::IsPublic.eq(true)),
            );

        Quizes::find_by_id(id)
            .join(JoinType::InnerJoin, quizes::Relation::SharedQuizes.def())
            .filter(condition)
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)
    }

    pub async fn get_all(
        &self,
        caller_id: Uuid,
        params: FilterQuizParams,
    ) -> Result<PaginatedResponse<QuizWithVisibility>> {
        let conn = self.db.get_connection().await;
        let mut condition = Condition::all().add(quizes::Column::IsDeleted.eq(false));
        let mut condition_visibility = Condition::any();

        if let Some(visibility) = params.visibility {
            if visibility.contains(&"publish".to_string()) {
                condition_visibility =
                    condition_visibility.add(quizes::Column::IsPublished.eq(true));
            }
            if visibility.contains(&"public".to_string()) {
                condition_visibility = condition_visibility.add(quizes::Column::IsPublic.eq(true));
            }
            if visibility.contains(&"owned".to_string()) {
                condition_visibility =
                    condition_visibility.add(quizes::Column::CreatorId.eq(caller_id));
            }
            if visibility.contains(&"shared".to_string()) {
                condition_visibility =
                    condition_visibility.add(shared_quizes::Column::UserId.eq(caller_id));
            }
        }
        if let Some(creator_id) = params.creator_id {
            condition = condition.add(
                Condition::any()
                    .add(quizes::Column::CreatorId.eq(creator_id))
                    .add(condition_visibility),
            );
        } else {
            condition = condition.add(condition_visibility);
        }

        let query = Quizes::find()
            .join(JoinType::InnerJoin, quizes::Relation::SharedQuizes.def())
            .filter(condition);

        // 🔹 Apply pagination (default: page=1, page_size=10)
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);

        let column = if let Some(sort_by) = params.sort_by {
            match sort_by.as_str() {
                "name" => quizes::Column::Name,
                "total_point" => quizes::Column::TotalPoint,
                "created_at" => quizes::Column::CreatedAt,
                "updated_at" => quizes::Column::UpdatedAt,
                "published_at" => quizes::Column::PublishAt,
                _ => quizes::Column::CreatedAt,
            }
        } else {
            quizes::Column::CreatedAt
        };

        let query = match &params.sort_direction {
            Some(direction) if direction == "asc" => query.order_by_asc(column),
            Some(direction) if direction == "desc" => query.order_by_asc(column),
            _ => query.order_by_desc(column),
        };

        let paginator = query.paginate(&conn, page_size);
        let total_pages = paginator.num_pages().await.unwrap_or(1);

        let quizzes = paginator
            .fetch_page(page - 1)
            .await
            .map_err(Error::QueryFailed)?;

        // Get shared quizzes for the caller to check "shared" status
        let shared_quiz_ids = SharedQuizes::find()
            .filter(shared_quizes::Column::UserId.eq(caller_id))
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .into_iter()
            .map(|shared| shared.quiz_id)
            .collect::<Vec<Uuid>>();

        // Transform each quiz model to include visibility information
        let quizzes_with_visibility = quizzes
            .into_iter()
            .map(|quiz| {
                let mut visibility = Vec::new();

                // Check each visibility type
                if quiz.is_published {
                    visibility.push("publish".to_string());
                }
                if quiz.is_public {
                    visibility.push("public".to_string());
                }
                if quiz.creator_id == caller_id {
                    visibility.push("owned".to_string());
                }
                if shared_quiz_ids.contains(&quiz.id) {
                    visibility.push("shared".to_string());
                }

                QuizWithVisibility { quiz, visibility }
            })
            .collect::<Vec<_>>();

        Ok(PaginatedResponse {
            total_pages,
            current_page: page,
            page_size,
            data: quizzes_with_visibility,
        })
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

    // pub async fn get_all_shared_quizzes_of_user(
    //     &self,
    //     user_id: Uuid,
    // ) -> Result<Vec<quizes::Model>> {
    //     let conn = self.db.get_connection().await;
    //     let user = Users::find_by_id(user_id)
    //         .one(&conn)
    //         .await
    //         .map_err(Error::QueryFailed)?
    //         .ok_or(Error::RecordNotFound)?;

    //     user.find_related(Quizes)
    //         .filter(quizes::Column::IsDeleted.eq(false))
    //         .all(&conn)
    //         .await
    //         .map_err(Error::QueryFailed)
    // }

    pub async fn get_all_shared_users_of_quiz(&self, quiz_id: Uuid) -> Result<Vec<UserModel>> {
        let conn = self.db.get_connection().await;
        let quiz = Quizes::find_by_id(quiz_id)
            .filter(quizes::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?;

        Ok(quiz
            .find_related(Users)
            .filter(users::Column::IsDeleted.eq(false))
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .into_iter()
            .map(|user| user.into())
            .collect::<Vec<_>>())
    }

    pub async fn is_shared_with(&self, quiz_id: Uuid, user_id: Uuid) -> Result<bool> {
        let conn = self.db.get_connection().await;
        let condition = Condition::all()
            .add(shared_quizes::Column::QuizId.eq(quiz_id))
            .add(shared_quizes::Column::UserId.eq(user_id));

        let shared_quiz = SharedQuizes::find()
            .filter(condition)
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?;

        Ok(shared_quiz.is_some())
    }
}
