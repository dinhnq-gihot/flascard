use {
    crate::{
        enums::{
            error::*,
            generic::{into_ok_response, PaginatedResponse},
        },
        models::test::{
            CreateTest, CreateTestResponse, CurrentTestState, QueryTestParams, ResolveResponse,
            ResolveTestingQuestion, TestModel, TestingAnswer, TestingQuestion, TestingQuiz,
            UpdateTestParams,
        },
        server::AppState,
        utils::helpers::check_test_status,
    },
    axum::{
        extract::{Path, Query, State},
        response::IntoResponse,
        Json,
    },
    std::sync::Arc,
    uuid::Uuid,
};

pub struct TestHandler;

impl TestHandler {
    pub async fn get_all(
        State(state): State<AppState>,
        Query(params): Query<QueryTestParams>,
    ) -> Result<impl IntoResponse> {
        let test_service = Arc::clone(&state.test_service);
        let quiz_service = Arc::clone(&state.quiz_service);
        let set_service = Arc::clone(&state.set_service);

        let PaginatedResponse {
            total_pages,
            current_page,
            page_size,
            data,
        } = test_service.get_all(params).await?;

        let mut res = Vec::<TestModel>::new();

        for (test, test_state) in data.into_iter() {
            let status = check_test_status(test.started_at, test.submitted_at);

            let quiz = quiz_service.get_by_id(test.quiz_id).await?;
            let set = set_service.get_by_id(quiz.set_id).await?;

            let test_info = TestModel {
                id: test.id,
                quiz: TestingQuiz {
                    id: quiz.id,
                    name: quiz.name,
                    set_id: set.id,
                    set_name: set.name,
                },
                status,
                score: test.score,
                created_at: test.created_at,
                started_at: test.started_at,
                submitted_at: test.submitted_at,
                max_duration: test.duration,
                remaining_time: test_state.remaining_time,
                current_state: None,
            };

            res.push(test_info);
        }

        Ok(into_ok_response(
            "Success".into(),
            Some(PaginatedResponse {
                total_pages,
                current_page,
                page_size,
                data: res,
            }),
        ))
    }

    pub async fn get_by_id(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let service = Arc::clone(&state.test_service);
        let quiz_service = Arc::clone(&state.quiz_service);
        let set_service = Arc::clone(&state.set_service);

        let (test, test_state) = service.get_one(id).await?;
        let quiz = quiz_service.get_by_id(test.quiz_id).await?;
        let set = set_service.get_by_id(quiz.set_id).await?;

        let status = check_test_status(test.started_at, test.submitted_at);

        let res = TestModel {
            id,
            quiz: TestingQuiz {
                id: quiz.id,
                name: quiz.name,
                set_id: set.id,
                set_name: set.name,
            },
            status,
            score: test.score,
            created_at: test.created_at,
            started_at: test.started_at,
            submitted_at: test.submitted_at,
            max_duration: test.duration,
            remaining_time: test_state.remaining_time,
            current_state: Some(CurrentTestState {
                current_question_id: test_state.current_quiz_question,
                completed_questions: test_state.completed_questions,
                spent_time_in_second: test.duration - test_state.remaining_time,
            }),
        };

        Ok(into_ok_response("Success".into(), Some(res)))
    }

    pub async fn create(
        State(state): State<AppState>,
        Json(payload): Json<CreateTest>,
    ) -> Result<impl IntoResponse> {
        let test_service = Arc::clone(&state.test_service);
        let quiz_service = Arc::clone(&state.quiz_service);
        let set_service = Arc::clone(&state.set_service);

        let test = test_service
            .create_one(payload.quiz_id, payload.max_duration)
            .await?;
        let quiz = quiz_service.get_by_id(test.quiz_id).await?;
        let set = set_service.get_by_id(quiz.set_id).await?;

        let res = CreateTestResponse {
            id: test.id,
            max_duration: test.duration,
            quiz: TestingQuiz {
                id: quiz.id,
                name: quiz.name,
                set_id: set.id,
                set_name: set.name,
            },
            status: "Not Started".into(),
            created_at: test.created_at,
        };

        Ok(into_ok_response("Created successfully".into(), Some(res)))
    }

    pub async fn start(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let test_service = Arc::clone(&state.test_service);
        let quiz_question_service = Arc::clone(&state.quiz_question_service);

        let now = chrono::Utc::now().naive_utc();

        let (test, test_state) = test_service.get_one(id).await?;
        if test.started_at.is_none() {
            // update state to started
            test_service
                .update_one(
                    id,
                    UpdateTestParams {
                        started_at: Some(now),
                        submitted_at: None,
                        resolved_count: None,
                        remaining_time: None,
                    },
                )
                .await?;
        }

        // get response
        let (quiz_question, quiz_question_answers) = quiz_question_service
            .get_by_id(test_state.current_quiz_question, test.quiz_id)
            .await?;
        let test_question_result = test_service
            .get_test_question_result(test.id, quiz_question.id)
            .await?;
        let (text_answer, selected_answer_ids, spent_time_in_second) =
            if let Some(test_question_result) = test_question_result {
                (
                    test_question_result.text_answer,
                    test_question_result.selected_answer_ids,
                    test_question_result.spent_time,
                )
            } else {
                (None, None, 0)
            };

        let res = TestingQuestion {
            id: quiz_question.id,
            content: quiz_question.question_content,
            answers: quiz_question_answers
                .into_iter()
                .map(|a| a.into())
                .collect::<Vec<TestingAnswer>>(),
            r#type: quiz_question.r#type,
            selected_answer_ids,
            spent_time_in_second,
            text_answer,
            next_question_id: quiz_question.next_question,
            previous_question_id: quiz_question.previous_question,
        };

        Ok(into_ok_response("Started successfully".into(), Some(res)))
    }

    pub async fn get_all_test_questions(
        State(state): State<AppState>,
    ) -> Result<impl IntoResponse> {
        Ok(())
    }

    pub async fn get_test_question(
        State(state): State<AppState>,
        Path(test_id): Path<Uuid>,
        Path(question_id): Path<Uuid>,
    ) -> Result<impl IntoResponse> {
        let test_service = Arc::clone(&state.test_service);
        let quiz_question_service = Arc::clone(&state.quiz_question_service);

        let (test, _) = test_service.get_one(test_id).await?;
        let (quiz_question, quiz_question_answers) = quiz_question_service
            .get_by_id(question_id, test.quiz_id)
            .await?;
        let test_question_result = test_service
            .get_test_question_result(test.id, quiz_question.id)
            .await?;
        let (text_answer, selected_answer_ids, spent_time_in_second) =
            if let Some(test_question_result) = test_question_result {
                (
                    test_question_result.text_answer,
                    test_question_result.selected_answer_ids,
                    test_question_result.spent_time,
                )
            } else {
                (None, None, 0)
            };

        let res = TestingQuestion {
            id: quiz_question.id,
            content: quiz_question.question_content,
            answers: quiz_question_answers
                .into_iter()
                .map(|a| a.into())
                .collect::<Vec<TestingAnswer>>(),
            r#type: quiz_question.r#type,
            selected_answer_ids,
            spent_time_in_second,
            text_answer,
            next_question_id: quiz_question.next_question,
            previous_question_id: quiz_question.previous_question,
        };

        Ok(into_ok_response("Success".into(), Some(res)))
    }

    // Khi ấn next/previous/thoát => chương trình lưu lại câu trả lời vào
    // test_result và ghi lại trạng thái của test là test_state
    pub async fn resolve_test_question(
        State(state): State<AppState>,
        Path(test_id): Path<Uuid>,
        Path(question_id): Path<Uuid>,
        Json(payload): Json<ResolveTestingQuestion>,
    ) -> Result<impl IntoResponse> {
        let test_service = Arc::clone(&state.test_service);
        let quiz_question_service = Arc::clone(&state.quiz_question_service);

        let remaining_time = payload.remainning_time;

        // Answer: create test_question_result ->
        let resolved_count = test_service
            .create_test_question_result(test_id, question_id, payload)
            .await?;

        let (test, _) = test_service
            .update_one(
                test_id,
                UpdateTestParams {
                    started_at: None,
                    submitted_at: None,
                    resolved_count: Some(resolved_count),
                    remaining_time: Some(remaining_time),
                },
            )
            .await?;

        let (question, _) = quiz_question_service
            .get_by_id(question_id, test.quiz_id)
            .await?;

        Ok(into_ok_response(
            "success".into(),
            Some(ResolveResponse {
                next_question_id: question.next_question,
            }),
        ))
    }

    pub async fn submit(State(state): State<AppState>) -> Result<impl IntoResponse> {
        Ok(())
    }

    pub async fn save_state(State(state): State<AppState>) -> Result<impl IntoResponse> {
        Ok(())
    }

    pub async fn result(State(state): State<AppState>) -> Result<impl IntoResponse> {
        Ok(())
    }

    pub async fn review_solution(State(state): State<AppState>) -> Result<impl IntoResponse> {
        Ok(())
    }
}
