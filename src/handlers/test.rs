use {
    crate::{
        enums::{
            error::*,
            generic::{into_ok_response, PaginatedResponse},
        },
        models::test::{
            CreateTest, CreateTestResponse, CurrentTestState, QueryTestParams, TestModel,
            TestingQuiz,
        },
        server::AppState,
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
            let mut status = String::new();
            if test.started_at.is_none() && !test.submitted {
                status = "Not Started".into();
            } else if test.started_at.is_some() && !test.submitted {
                status = "In Progess".into();
            } else if test.submitted {
                status = "Completed".into();
            }

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

        let mut status = String::new();
        if test.started_at.is_none() && !test.submitted {
            status = "Not Started".into();
        }
        if test.started_at.is_some() && !test.submitted {
            status = "In Progess".into();
        }
        if test.submitted {
            status = "Completed".into();
        }

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
        let service = Arc::clone(&state.test_service);
        
        // update state

        // 
        
        Ok(())
    }

    pub async fn get_all_test_questions(
        State(state): State<AppState>,
    ) -> Result<impl IntoResponse> {
        Ok(())
    }

    pub async fn get_test_question(State(state): State<AppState>) -> Result<impl IntoResponse> {
        Ok(())
    }

    pub async fn answer_test_question(State(state): State<AppState>) -> Result<impl IntoResponse> {
        Ok(())
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
