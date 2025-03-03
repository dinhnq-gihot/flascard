use {
    crate::{
        db::db::Database,
        entities::{
            answers,
            prelude::{Answers, Questions},
            questions,
        },
        enums::{error::*, generic::PaginatedResponse},
        models::qna::{CreateQnARequest, QueryQuestionParams, UpdateAnswerRequest},
    },
    chrono::Utc,
    sea_orm::{
        ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter,
        QueryOrder, Set,
    },
    std::sync::Arc,
    uuid::Uuid,
};

pub struct QnAService {
    db: Arc<Database>,
}

impl QnAService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn create(
        self,
        payload: CreateQnARequest,
        creator_id: Uuid,
    ) -> Result<(questions::Model, Vec<answers::Model>)> {
        let conn = self.db.get_connection().await;

        let question = questions::ActiveModel {
            content: Set(payload.content),
            r#type: Set(payload.r#type),
            set_id: Set(payload.set_id),
            creator_id: Set(creator_id),
            ..Default::default()
        }
        .insert(&conn)
        .await
        .map_err(Error::InsertFailed)?;

        let mut created_answers = Vec::new();
        for a in payload.answers.iter() {
            let answer = answers::ActiveModel {
                content: Set(a.content.clone()),
                is_correct: Set(a.is_correct),
                question_id: Set(question.id),
                ..Default::default()
            }
            .insert(&conn)
            .await
            .map_err(Error::InsertFailed)?;

            created_answers.push(answer);
        }

        Ok((question, created_answers))
    }

    pub async fn get_by_id(self, id: Uuid) -> Result<(questions::Model, Vec<answers::Model>)> {
        let conn = self.db.get_connection().await;
        let question = Questions::find_by_id(id)
            .filter(questions::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?;

        let answers = question
            .find_related(Answers)
            .filter(answers::Column::IsDeleted.eq(false))
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)?;

        Ok((question, answers))
    }

    pub async fn get_all(
        self,
        params: QueryQuestionParams,
    ) -> Result<PaginatedResponse<questions::Model>> {
        let conn = self.db.get_connection().await;

        let mut query = Questions::find().filter(questions::Column::IsDeleted.eq(false));

        if let Some(content) = params.content {
            query = query.filter(questions::Column::Content.contains(&content));
        }
        if let Some(r#type) = params.r#type {
            query = query.filter(questions::Column::Type.eq(r#type));
        }

        // 🔹 Apply sorting (default: created_at DESC)
        query = match &params.sort_by {
            Some(sort_by) => {
                let column = match sort_by.as_str() {
                    "content" => questions::Column::Content,
                    "type" => questions::Column::Type,
                    _ => questions::Column::CreatedAt,
                };

                match &params.sort_direction {
                    Some(direction) if direction == "asc" => query.order_by_asc(column),
                    _ => query.order_by_desc(column),
                }
            }
            None => query.order_by_desc(questions::Column::CreatedAt),
        };

        // 🔹 Apply pagination (default: page=1, page_size=10)
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);

        let paginator = query.paginate(&conn, page_size);
        let total_pages = paginator.num_pages().await.unwrap_or(1);

        let res = paginator
            .fetch_page(page - 1)
            .await
            .map_err(Error::QueryFailed)?;

        Ok(PaginatedResponse {
            total_pages,
            current_page: page,
            page_size,
            data: res,
        })
    }

    pub async fn update_question(
        self,
        id: Uuid,
        content: Option<String>,
    ) -> Result<Option<questions::Model>> {
        let conn = self.db.get_connection().await;
        let mut question: questions::ActiveModel = Questions::find_by_id(id)
            .filter(questions::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?
            .into();

        let mut updated = false;
        if let Some(c) = content {
            question.content = Set(c);
            updated = true;
        }

        if updated {
            question.updated_at = Set(Utc::now().naive_utc());
            Ok(Some(
                question.update(&conn).await.map_err(Error::UpdateFailed)?,
            ))
        } else {
            Ok(None)
        }
    }

    pub async fn update_answer(
        self,
        id: Uuid,
        payload: UpdateAnswerRequest,
    ) -> Result<Option<answers::Model>> {
        let conn = self.db.get_connection().await;
        let mut answer: answers::ActiveModel = Answers::find_by_id(id)
            .filter(answers::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?
            .into();

        let mut updated = false;
        if let Some(c) = payload.content {
            answer.content = Set(c);
            updated = true;
        }
        if let Some(c) = payload.is_correct {
            answer.is_correct = Set(c);
            updated = true;
        }

        if updated {
            answer.updated_at = Set(Utc::now().naive_utc());
            Ok(Some(
                answer.update(&conn).await.map_err(Error::UpdateFailed)?,
            ))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_question(self, id: Uuid) -> Result<()> {
        let conn = self.db.get_connection().await;
        let mut question: questions::ActiveModel = Questions::find_by_id(id)
            .filter(questions::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?
            .into();
        question.is_deleted = Set(true);

        question.update(&conn).await.map_err(Error::DeleteFailed)?;

        Ok(())
    }

    pub async fn delete_answer(self, id: Uuid) -> Result<()> {
        let conn = self.db.get_connection().await;
        let mut answer: answers::ActiveModel = Answers::find_by_id(id)
            .filter(answers::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?
            .into();

        answer.is_deleted = Set(true);

        answer.update(&conn).await.map_err(Error::DeleteFailed)?;

        Ok(())
    }
}
