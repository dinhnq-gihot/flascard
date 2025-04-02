use {
    crate::{
        db::db_connection::Database,
        entities::{
            prelude::{TestResults, Tests},
            sea_orm_active_enums::StatusEnum,
            test_results, tests,
        },
        enums::{error::*, generic::PaginatedResponse},
        models::test::{QueryTestParams, UpdateTestParams, UpdateTestingResultResquest},
    },
    sea_orm::{
        ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
        TransactionTrait,
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
        params: UpdateTestParams,
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

        if let Some(started_at) = params.started_at {
            existing_test.started_at = Set(Some(started_at));
            updated = true;
        }
        if let Some(submitted_at) = params.submitted_at {
            existing_test.submitted_at = Set(Some(submitted_at));
            updated = true;
        }
        if let Some(current_testing_quiz_question) = params.current_testing_quiz_question {
            existing_test.current_quiz_question_id = Set(current_testing_quiz_question);
            updated = true;
        }
        if let Some(remaining_time) = params.remaining_time {
            existing_test.remaining_time = Set(remaining_time);
            updated = true;
        }
        if let Some(resolved_count) = params.resolved_count {
            existing_test.completed_questions = Set(resolved_count);
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

    pub async fn create_all_test_question_results(
        &self,
        test_id: Uuid,
        quiz_question_ids: Vec<Uuid>,
    ) -> Result<Vec<test_results::Model>> {
        let txn = self
            .db
            .get_connection()
            .await
            .begin()
            .await
            .map_err(Error::BeginTransactionFailed)?;

        let test_results = quiz_question_ids
            .into_iter()
            .map(|id| {
                test_results::ActiveModel {
                    test_id: Set(test_id),
                    quiz_question_id: Set(id),
                    ..Default::default()
                }
            })
            .collect::<Vec<_>>();

        let res = TestResults::insert_many(test_results)
            .exec_with_returning_many(&txn)
            .await
            .map_err(Error::InsertFailed)?;

        txn.commit().await.map_err(|e| Error::Anyhow(e.into()))?;

        Ok(res)
    }

    pub async fn update_test_result(
        &self,
        test_id: Uuid,
        test_result_id: Uuid,
        payload: UpdateTestingResultResquest,
    ) -> Result<Option<test_results::Model>> {
        let txn = self
            .db
            .get_connection()
            .await
            .begin()
            .await
            .map_err(Error::BeginTransactionFailed)?;

        let mut test_result: test_results::ActiveModel =
            test_results::Entity::find_by_id(test_result_id)
                .filter(test_results::Column::TestId.eq(test_id))
                .one(&txn)
                .await
                .map_err(Error::QueryFailed)?
                .ok_or(Error::RecordNotFound)?
                .into();
        let mut updated = false;

        if let Some(selected_answers_ids) = payload.selected_answer_ids {
            test_result.selected_answer_ids = Set(Some(selected_answers_ids));
            updated = true;
        }
        if let Some(spent_time) = payload.spent_time_in_second {
            test_result.spent_time = Set(spent_time);
            updated = true;
        }
        if let Some(text_answer) = payload.text_answer {
            test_result.text_answer = Set(Some(text_answer));
            updated = true;
        }

        let res = if updated {
            Some(
                test_result
                    .update(&txn)
                    .await
                    .map_err(Error::UpdateFailed)?,
            )
        } else {
            None
        };

        txn.commit().await.map_err(|e| Error::Anyhow(e.into()))?;
        Ok(res)
    }
}
