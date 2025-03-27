use {
    crate::{
        enums::{error::*, generic::PaginatedResponse},
        models::{
            quiz_question::QuizQuestionResponse,
            test::{
                CreateTest, CreateTestResponse, CurrentTestState, QueryTestParams,
                ResolveTestingQuestion, TestResponse, TestingAnswer, TestingQuestion, TestingQuiz,
                UpdateTestParams,
            },
        },
        repositories::test::TestRepository,
        services::traits::{
            quiz_question_trait::QuizQuestionService, quiz_trait::QuizService,
            set_trait::SetService, test_trait::TestService,
        },
        utils::helpers::check_test_status,
    },
    async_trait::async_trait,
    std::sync::Arc,
    uuid::Uuid,
};

pub struct TestServiceImpl {
    test_repository: Arc<TestRepository>,
    quiz_service: Arc<dyn QuizService>,
    quiz_question_service: Arc<dyn QuizQuestionService>,
    set_service: Arc<dyn SetService>,
}

impl TestServiceImpl {
    pub fn new(
        test_repository: Arc<TestRepository>,
        quiz_service: Arc<dyn QuizService>,
        quiz_question_service: Arc<dyn QuizQuestionService>,
        set_service: Arc<dyn SetService>,
    ) -> Self {
        Self {
            test_repository,
            quiz_service,
            set_service,
            quiz_question_service,
        }
    }
}

#[async_trait]
impl TestService for TestServiceImpl {
    async fn get_all(
        &self,
        caller_id: Uuid,
        params: QueryTestParams,
    ) -> Result<PaginatedResponse<TestResponse>> {
        let PaginatedResponse {
            total_pages,
            current_page,
            page_size,
            data,
        } = self.test_repository.get_all(caller_id, params).await?;

        let mut res = Vec::<TestResponse>::new();

        for (test, test_state) in data.into_iter() {
            let status = check_test_status(test.started_at, test.submitted_at);

            let quiz = self.quiz_service.get_by_id(test.quiz_id).await?;
            let set = self.set_service.get_by_id(quiz.set_id).await?;

            let test_info = TestResponse {
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

        Ok(PaginatedResponse {
            total_pages,
            current_page,
            page_size,
            data: res,
        })
    }

    async fn get_by_id(&self, caller_id: Uuid, test_id: Uuid) -> Result<TestResponse> {
        let (test, test_state) = self.test_repository.get_one(test_id).await?;
        let quiz = self.quiz_service.get_by_id(caller_id, test.quiz_id).await?;
        let set = self.set_service.get_by_id(quiz.set_id).await?;

        let status = check_test_status(test.started_at, test.submitted_at);

        Ok(TestResponse {
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
            current_state: Some(CurrentTestState {
                current_question_id: test_state.current_quiz_question,
                completed_questions: test_state.completed_questions,
                spent_time_in_second: test.duration - test_state.remaining_time,
            }),
        })
    }

    async fn create(&self, caller_id: Uuid, payload: CreateTest) -> Result<CreateTestResponse> {
        // tạo 1 đối tượng Test với quiz_id và duration
        let test = self
            .test_repository
            .create_one(payload.quiz_id, payload.max_duration)
            .await?;

        // lấy quiz và set để trả về cho response
        let quiz = self.quiz_service.get_by_id(caller_id, test.quiz_id).await?;
        let set = self.set_service.get_by_id(quiz.set_id).await?;

        Ok(CreateTestResponse {
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
        })
    }

    async fn start(&self, test_id: Uuid) -> Result<TestingQuestion> {
        let now = chrono::Utc::now().naive_utc();

        // Lấy test và test-state để làm
        let (test, test_state) = self.test_repository.get_one(test_id).await?;

        // Nếu test chưa được bắt đầu thì cập nhật trạng thái thành started
        if test.started_at.is_none() {
            // update state to started
            self.test_repository
                .update_one(
                    test_id,
                    UpdateTestParams {
                        started_at: Some(now),
                        submitted_at: None,
                        current_testing_quiz_question: None,
                        resolved_count: None,
                        remaining_time: None,
                    },
                )
                .await?;
        }

        // lấy câu hỏi quiz hiện tại trong test_state
        let QuizQuestionResponse {
            question: quiz_question,
            answers: quiz_question_answers,
        } = self
            .quiz_question_service
            .get_by_id(test_state.current_quiz_question, test.quiz_id)
            .await?;

        // lấy câu trả lời của quiz ở trên nếu có => để trả về cho FE show kết quả user
        // đã làm
        let testing_question_result = self
            .test_repository
            .get_test_question_result(test.id, quiz_question.id)
            .await?;
        let (text_answer, selected_answer_ids, spent_time_in_second) =
            if let Some(test_question_result) = testing_question_result {
                (
                    test_question_result.text_answer,
                    test_question_result.selected_answer_ids,
                    test_question_result.spent_time,
                )
            } else {
                (None, None, 0)
            };

        Ok(TestingQuestion {
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
        })
    }

    async fn get_test_question(&self, test_id: Uuid, question_id: Uuid) -> Result<TestingQuestion> {
        let (test, _) = self.test_repository.get_one(test_id).await?;
        let QuizQuestionResponse {
            question: quiz_question,
            answers: quiz_question_answers,
        } = self
            .quiz_question_service
            .get_by_id(question_id, test.quiz_id)
            .await?;
        let test_question_result = self
            .test_repository
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

        Ok(TestingQuestion {
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
        })
    }

    // Khi ấn next/previous/chọn câu bất kỳ/thoát => chương trình lưu lại câu trả
    // lời vào test_result và ghi lại câu quiz đang làm vào test_state
    async fn resolve_test_question(
        &self,
        test_id: Uuid,
        question_id: Uuid,
        payload: ResolveTestingQuestion,
    ) -> Result<()> {
        let remaining_time = payload.remainning_time;

        // Answer: create test_question_result ->
        let resolved_count = self
            .test_repository
            .create_test_question_result(test_id, question_id, payload)
            .await?;

        let (test, _) = self
            .test_repository
            .update_one(
                test_id,
                UpdateTestParams {
                    started_at: None,
                    submitted_at: None,
                    current_testing_quiz_question: Some(question_id),
                    resolved_count: Some(resolved_count),
                    remaining_time: Some(remaining_time),
                },
            )
            .await?;

        let QuizQuestionResponse {
            question,
            answers: _,
        } = self
            .quiz_question_service
            .get_by_id(question_id, test.quiz_id)
            .await?;

        // TODO next

        Ok(())
    }

    // Nhấn nút submit => chương trình chấm điểm các test result
    async fn submit(&self, test_id: Uuid) -> Result<()> {
        Ok(())
    }

    async fn result(&self, test_id: Uuid) -> Result<()> {
        Ok(())
    }

    async fn review_solution(&self, test_id: Uuid, question_id: Uuid) -> Result<()> {
        Ok(())
    }
}
