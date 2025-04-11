use {
    crate::{
        debug,
        entities::{
            sea_orm_active_enums::{QuestionTypeEnum, StatusEnum},
            test_question_results, tests,
        },
        enums::{error::*, generic::PaginatedResponse},
        error,
        models::{
            quiz::QuestionCounts,
            test::{
                CreateTest, QueryTestParams, ResolveTestRequest, ResultResponse, SolutionResponse,
                TestingQuestion, UpdateTest,
            },
        },
        repositories::test::TestRepository,
        services::traits::{
            quiz_question_trait::QuizQuestionService, quiz_trait::QuizService,
            test_trait::TestService,
        },
        utils::helpers::total_question_count,
    },
    async_trait::async_trait,
    chrono::Utc,
    std::{collections::HashSet, sync::Arc},
    uuid::Uuid,
};

pub struct TestServiceImpl {
    test_repository: Arc<TestRepository>,
    quiz_service: Arc<dyn QuizService>,
    quiz_question_service: Arc<dyn QuizQuestionService>,
    // set_service: Arc<dyn SetService>,
}

impl TestServiceImpl {
    pub fn new(
        test_repository: Arc<TestRepository>,
        quiz_service: Arc<dyn QuizService>,
        quiz_question_service: Arc<dyn QuizQuestionService>,
        // set_service: Arc<dyn SetService>,
    ) -> Self {
        Self {
            test_repository,
            quiz_service,
            // set_service,
            quiz_question_service,
        }
    }
}

#[async_trait]
impl TestService for TestServiceImpl {
    async fn create_one(&self, caller_id: Uuid, payload: CreateTest) -> Result<tests::Model> {
        let quiz = self
            .quiz_service
            .get_by_id(caller_id, payload.quiz_id)
            .await?;
        let quiz_question_ids = self
            .quiz_question_service
            .get_all(caller_id, quiz.id)
            .await?
            .iter()
            .map(|q| q.question.id)
            .collect::<Vec<_>>();

        let total_questions = total_question_count(
            serde_json::from_value::<QuestionCounts>(quiz.question_counts)
                .map_err(|e| Error::Anyhow(e.into()))?,
        );
        let res = self
            .test_repository
            .create_one(
                quiz.id,
                caller_id,
                quiz.duration,
                quiz_question_ids[0],
                total_questions,
            )
            .await?;

        let test_question_results = self
            .test_repository
            .create_test_question_results(res.id, quiz_question_ids)
            .await?;

        debug!("test_question_results: {test_question_results:?}");

        Ok(res)
    }

    async fn get_all(
        &self,
        caller_id: Uuid,
        params: QueryTestParams,
    ) -> Result<PaginatedResponse<tests::Model>> {
        self.test_repository.get_all_tests(caller_id, params).await
    }

    async fn get_by_id(&self, caller_id: Uuid, test_id: Uuid) -> Result<tests::Model> {
        self.test_repository.get_by_id(caller_id, test_id).await
    }

    async fn start_one(&self, caller_id: Uuid, test_id: Uuid) -> Result<TestingQuestion> {
        // kiểm tra status => nếu submitted/abandoned thì trả về lỗi
        // cập nhật start time
        // trả về current test question

        let test = self.get_by_id(caller_id, test_id).await?;
        if test.status == StatusEnum::Submitted || test.status == StatusEnum::Abandoned {
            return Err(Error::TestEnded);
        }

        let test = self
            .test_repository
            .update_one(
                caller_id,
                test_id,
                UpdateTest {
                    started_at: Some(Utc::now().naive_utc()),
                    status: Some(StatusEnum::InProgess),
                    ..Default::default()
                },
            )
            .await?
            .unwrap();

        self.get_testing_question(caller_id, test_id, test.current_quiz_question_id)
            .await
    }

    async fn get_testing_question(
        &self,
        caller_id: Uuid,
        test_id: Uuid,
        quiz_question_id: Uuid,
    ) -> Result<TestingQuestion> {
        let test = self.test_repository.get_by_id(caller_id, test_id).await?;
        if test.status == StatusEnum::Submitted || test.status == StatusEnum::Abandoned {
            return Err(Error::TestEnded);
        }
        let test_answers = self
            .test_repository
            .get_test_answers(test_id, quiz_question_id)
            .await?;

        let mut testing_question: TestingQuestion = self
            .quiz_question_service
            .get_by_id(caller_id, test.quiz_id, quiz_question_id)
            .await?
            .into();

        if testing_question.r#type != QuestionTypeEnum::TextFill {
            testing_question.answers = vec![];
        }
        testing_question.user_answers = test_answers;

        Ok(testing_question)
    }

    async fn get_all_testing_question_results(
        &self,
        test_id: Uuid,
    ) -> Result<Vec<test_question_results::Model>> {
        self.test_repository
            .get_all_test_question_result(test_id)
            .await
    }

    async fn resolve_testing_question(
        &self,
        caller_id: Uuid,
        test_id: Uuid,
        quiz_question_id: Uuid,
        payloads: ResolveTestRequest,
    ) -> Result<Option<tests::Model>> {
        // kiểm tra status => nếu submitted/abandoned thì trả về lỗi
        let test = self.get_by_id(caller_id, test_id).await?;
        if test.status == StatusEnum::Submitted || test.status == StatusEnum::Abandoned {
            return Err(Error::TestEnded);
        }

        let updated_test_result = self
            .test_repository
            .save_test_answers(test_id, quiz_question_id, payloads.save_test_answers)
            .await
            .inspect_err(|e| {
                error!("{}", e.to_string());
            })?;
        debug!("updated test result {:?}", updated_test_result);

        let updated_test = self
            .test_repository
            .update_one(
                caller_id,
                test_id,
                UpdateTest {
                    current_testing_quiz_question: Some(quiz_question_id),
                    resolved_count: Some(test.completed_questions + 1),
                    remaining_time: Some(payloads.remaining_time),
                    ..Default::default()
                },
            )
            .await?;

        Ok(updated_test)
    }

    async fn submit_one(
        &self,
        caller_id: Uuid,
        test_id: Uuid,
    ) -> Result<Vec<test_question_results::Model>> {
        // kiểm tra status => nếu submitted/abandoned thì trả về lỗi
        let test = self.get_by_id(caller_id, test_id).await?;
        if test.status == StatusEnum::Submitted || test.status == StatusEnum::Abandoned {
            return Err(Error::TestEnded);
        }

        let quiz_qnas = self
            .quiz_question_service
            .get_all(caller_id, test.quiz_id)
            .await?;

        // loop qua cac quiz question va lấy các đáp án đúng trong question
        // lấy selected answer của user từ quiz question
        // - Nếu là trắc nghiệm: so sánh 2 list có bằng nhau hay không
        // - Nếu là câu trả lời tự viết: todo
        let mut results = Vec::new();
        let mut total_score = 0;

        for quiz_qna in quiz_qnas.into_iter() {
            if quiz_qna.question.r#type != QuestionTypeEnum::TextFill {
                let correct_answer_ids = quiz_qna
                    .answers
                    .iter()
                    .filter(|a| a.is_answer)
                    .map(|a| a.id)
                    .collect::<HashSet<Uuid>>();
                let selected_answer_ids = self
                    .test_repository
                    .get_test_answers(test_id, quiz_qna.question.id)
                    .await?
                    .iter()
                    .map(|s| s.selected_answer_id.unwrap())
                    .collect::<HashSet<Uuid>>();

                results.push((
                    quiz_qna.question.id,
                    correct_answer_ids == selected_answer_ids,
                ));
                if correct_answer_ids == selected_answer_ids {
                    total_score += quiz_qna.question.point;
                }
            } else {
                // todo: implement check text fill
                let answers = self
                    .test_repository
                    .get_test_answers(test_id, quiz_qna.question.id)
                    .await?;
                if answers.len() == 1 {
                    results.push((quiz_qna.question.id, true));
                    total_score += quiz_qna.question.point;
                }
            }
        }

        let updated_test_question_results = self
            .test_repository
            .update_test_question_results(test_id, results)
            .await?;

        let updated_test = self
            .test_repository
            .update_one(
                caller_id,
                test_id,
                UpdateTest {
                    submitted_at: Some(Utc::now().naive_utc()),
                    status: Some(StatusEnum::Submitted),
                    score: Some(total_score),
                    ..Default::default()
                },
            )
            .await?
            .unwrap();
        debug!("updated test {:?}", updated_test);

        Ok(updated_test_question_results)
    }

    async fn result(&self, caller_id: Uuid, test_id: Uuid) -> Result<ResultResponse> {
        let test = self.test_repository.get_by_id(caller_id, test_id).await?;
        if test.status != StatusEnum::Submitted && test.status != StatusEnum::Abandoned {
            return Err(Error::TestNotEnd);
        }

        let test_result = self
            .test_repository
            .get_all_test_question_result(test.id)
            .await?;

        Ok(ResultResponse {
            test,
            result: test_result,
        })
    }

    async fn review_solution(
        &self,
        caller_id: Uuid,
        test_id: Uuid,
        quiz_question_id: Uuid,
    ) -> Result<SolutionResponse> {
        let test = self.test_repository.get_by_id(caller_id, test_id).await?;

        if test.status != StatusEnum::Submitted && test.status != StatusEnum::Abandoned {
            return Err(Error::TestNotEnd);
        }

        let test_result = self
            .test_repository
            .get_test_question_result(test_id, quiz_question_id)
            .await?;
        let test_answers = self
            .test_repository
            .get_test_answers(test_id, quiz_question_id)
            .await?;

        let mut selected_answer_ids = Vec::new();
        let mut text_answer = None::<String>;
        let mut spent_time = 0;

        for answer in test_answers.into_iter() {
            if let Some(value) = answer.selected_answer_id {
                selected_answer_ids.push(value);
            }
            if let Some(value) = answer.text_answer {
                text_answer = Some(value);
            }
            spent_time = answer.spent_time;
        }

        let quiz_qna = self
            .quiz_question_service
            .get_by_id(caller_id, test.quiz_id, quiz_question_id)
            .await?;

        Ok(SolutionResponse {
            solution: quiz_qna,
            text_answer,
            selected_answer_ids,
            is_correct: test_result.is_correct,
            spent_time,
        })
    }
}
