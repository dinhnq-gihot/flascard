use {
    crate::{
        db::db_connection::Database,
        entities::{
            prelude::{TestAnswers, TestQuestionResults, Tests},
            sea_orm_active_enums::StatusEnum,
            test_answers, test_question_results, tests,
        },
        enums::{error::*, generic::PaginatedResponse},
        models::test::{QueryTestParams, SaveTestAnswer, UpdateTest},
    },
    sea_orm::{
        sea_query::OnConflict, ActiveModelTrait, ColumnTrait, Condition, EntityTrait,
        PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait, TryIntoModel,
    },
    std::sync::Arc,
    uuid::Uuid,
};

pub struct TestRepository {
    db: Arc<Database>,
}

impl TestRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn create_one(
        &self,
        quiz_id: Uuid,
        user_id: Uuid,
        duration: i32,
        first_quiz_question_id: Uuid,
        total_question: i32,
    ) -> Result<tests::Model> {
        let conn = self.db.get_connection().await;

        tests::ActiveModel {
            quiz_id: Set(quiz_id),
            user_id: Set(user_id),
            duration: Set(duration),
            current_quiz_question_id: Set(first_quiz_question_id),
            remaining_time: Set(duration),
            total_question: Set(total_question),
            ..Default::default()
        }
        .insert(&conn)
        .await
        .map_err(Error::InsertFailed)
    }

    pub async fn get_all_tests(
        &self,
        caller_id: Uuid,
        params: QueryTestParams,
    ) -> Result<PaginatedResponse<tests::Model>> {
        let conn = self.db.get_connection().await;
        let query = Tests::find().filter(tests::Column::UserId.eq(caller_id));

        let column = if let Some(sort_by) = params.sort_by {
            match sort_by.as_str() {
                "score" => tests::Column::Score,
                "started_at" => tests::Column::StartedAt,
                "submitted_at" => tests::Column::SubmittedAt,
                "duration" => tests::Column::Duration,
                "status" => tests::Column::Status,
                _ => tests::Column::CreatedAt,
            }
        } else {
            tests::Column::CreatedAt
        };

        let query = match &params.sort_order {
            Some(direction) if direction == "asc" => query.order_by_asc(column),
            Some(direction) if direction == "desc" => query.order_by_asc(column),
            _ => query.order_by_desc(column),
        };

        // 🔹 Apply pagination (default: page=1, page_size=10)
        let page = params.page.unwrap_or(1);
        let page_size = 10;

        let paginator = query.paginate(&conn, page_size);
        let total_pages = paginator.num_pages().await.unwrap_or(1);

        let tests = paginator
            .fetch_page(page - 1)
            .await
            .map_err(Error::QueryFailed)?;

        Ok(PaginatedResponse {
            total_pages,
            current_page: page,
            page_size,
            data: tests,
        })
    }

    pub async fn get_by_id(&self, caller_id: Uuid, test_id: Uuid) -> Result<tests::Model> {
        let conn = self.db.get_connection().await;

        Tests::find_by_id(test_id)
            .filter(tests::Column::UserId.eq(caller_id))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)
    }

    pub async fn update_one(
        &self,
        caller_id: Uuid,
        test_id: Uuid,
        payload: UpdateTest,
    ) -> Result<Option<tests::Model>> {
        let conn = self.db.get_connection().await;

        let mut existing_test: tests::ActiveModel = Tests::find_by_id(test_id)
            .filter(tests::Column::UserId.eq(caller_id))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?
            .into();

        let mut updated = false;

        if let Some(started_at) = payload.started_at {
            existing_test.started_at = Set(Some(started_at));
            updated = true;
        }
        if let Some(submitted_at) = payload.submitted_at {
            existing_test.submitted_at = Set(Some(submitted_at));
            updated = true;
        }
        if let Some(current_testing_quiz_question) = payload.current_testing_quiz_question {
            existing_test.current_quiz_question_id = Set(current_testing_quiz_question);
            updated = true;
        }
        if let Some(remaining_time) = payload.remaining_time {
            existing_test.remaining_time = Set(remaining_time);
            updated = true;
        }
        if let Some(resolved_count) = payload.resolved_count {
            existing_test.completed_questions = Set(resolved_count);
            updated = true;
        }
        if let Some(status) = payload.status {
            existing_test.status = Set(status);
            updated = true;
        }

        if updated {
            Ok(Some(
                existing_test
                    .update(&conn)
                    .await
                    .map_err(Error::UpdateFailed)?,
            ))
        } else {
            Ok(None)
        }
    }

    pub async fn check_test_status(
        &self,
        caller_id: Uuid,
        test_id: Uuid,
        status: StatusEnum,
    ) -> Result<bool> {
        let test = self.get_by_id(caller_id, test_id).await?;
        if test.status == status {
            return Ok(true);
        }
        Ok(false)
    }

    pub async fn save_test_answers(
        &self,
        test_id: Uuid,
        quiz_question_id: Uuid,
        payloads: Vec<SaveTestAnswer>,
    ) -> Result<Vec<test_answers::Model>> {
        let txn = self
            .db
            .get_connection()
            .await
            .begin()
            .await
            .map_err(Error::BeginTransactionFailed)?;

        let mut res = Vec::new();

        for payload in payloads.into_iter() {
            let json = serde_json::to_value(payload).map_err(|e| Error::Anyhow(e.into()))?;
            let mut test_result =
                test_answers::ActiveModel::from_json(json).map_err(|e| Error::Anyhow(e.into()))?;
            test_result.test_id = Set(test_id);
            test_result.quiz_question_id = Set(quiz_question_id);

            let model = test_result
                .save(&txn)
                .await
                .map_err(Error::InsertFailed)?
                .try_into_model()
                .map_err(Error::IntoModelError)?;

            res.push(model);
        }

        txn.commit().await.map_err(|e| Error::Anyhow(e.into()))?;

        Ok(res)
    }

    pub async fn get_test_answers(
        &self,
        test_id: Uuid,
        quiz_question_id: Uuid,
    ) -> Result<Vec<test_answers::Model>> {
        let conn = self.db.get_connection().await;

        TestAnswers::find()
            .filter(
                Condition::all()
                    .add(test_answers::Column::TestId.eq(test_id))
                    .add(test_answers::Column::QuizQuestionId.eq(quiz_question_id)),
            )
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)
    }

    pub async fn create_test_question_results(
        &self,
        test_id: Uuid,
        quiz_question_ids: Vec<Uuid>,
    ) -> Result<Vec<test_question_results::Model>> {
        let txn = self
            .db
            .get_connection()
            .await
            .begin()
            .await
            .map_err(Error::BeginTransactionFailed)?;

        let result_ams = quiz_question_ids
            .into_iter()
            .map(|quiz_question_id| {
                test_question_results::ActiveModel {
                    test_id: Set(test_id),
                    quiz_question_id: Set(quiz_question_id),
                    ..Default::default()
                }
            })
            .collect::<Vec<_>>();

        let res = test_question_results::Entity::insert_many(result_ams)
            .exec_with_returning_many(&txn)
            .await
            .map_err(Error::QueryFailed)?;

        txn.commit().await.map_err(Error::CommitTransactionFailed)?;

        Ok(res)
    }

    pub async fn update_test_question_results(
        &self,
        test_id: Uuid,
        results: Vec<(Uuid, bool)>,
    ) -> Result<Vec<test_question_results::Model>> {
        let txn = self
            .db
            .get_connection()
            .await
            .begin()
            .await
            .map_err(Error::BeginTransactionFailed)?;

        let result_ams = results
            .into_iter()
            .map(|(quiz_question_id, is_correct)| {
                test_question_results::ActiveModel {
                    test_id: Set(test_id),
                    quiz_question_id: Set(quiz_question_id),
                    is_correct: Set(Some(is_correct)),
                    ..Default::default()
                }
            })
            .collect::<Vec<_>>();

        let on_conflict = OnConflict::columns([
            test_question_results::Column::TestId,
            test_question_results::Column::QuizQuestionId,
        ])
        .update_column(test_question_results::Column::IsCorrect)
        .to_owned();

        let res = TestQuestionResults::insert_many(result_ams)
            .on_conflict(on_conflict)
            .exec_with_returning_many(&txn)
            .await
            .map_err(Error::InsertFailed)?;

        txn.commit().await.map_err(Error::CommitTransactionFailed)?;

        Ok(res)
    }

    pub async fn get_test_question_result(
        &self,
        test_id: Uuid,
        quiz_question_id: Uuid,
    ) -> Result<test_question_results::Model> {
        let conn = self.db.get_connection().await;

        TestQuestionResults::find()
            .filter(
                Condition::all().add(
                    test_question_results::Column::TestId
                        .eq(test_id)
                        .add(test_question_results::Column::QuizQuestionId.eq(quiz_question_id)),
                ),
            )
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)
    }

    pub async fn get_all_test_question_result(
        &self,
        test_id: Uuid,
    ) -> Result<Vec<test_question_results::Model>> {
        let conn = self.db.get_connection().await;

        TestQuestionResults::find()
            .filter(Condition::all().add(test_question_results::Column::TestId.eq(test_id)))
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)
    }

    
}
