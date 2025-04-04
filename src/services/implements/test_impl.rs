use {
    crate::{
        debug,
        entities::{
            sea_orm_active_enums::{QuestionTypeEnum, StatusEnum},
            test_answers, test_question_results, tests,
        },
        enums::{error::*, generic::PaginatedResponse},
        error,
        models::{
            quiz::{self, QuestionCounts},
            quiz_question::QuizQuestionResponse,
            test::{
                CreateTest, CreateTestResponse, CurrentTestState, QueryTestParams,
                ResolveTestRequest, SaveTestAnswer, TestResponse, TestingAnswer, TestingQuestion,
                TestingQuiz, UpdateTest,
            },
        },
        repositories::test::TestRepository,
        services::traits::{
            quiz_question_trait::QuizQuestionService, quiz_trait::QuizService,
            set_trait::SetService, test_trait::TestService,
        },
        utils::helpers::{check_test_status, total_question_count},
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
        self.test_repository
            .get_all_tests(caller_id, params)
            .await?;
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
            .map_err(|e| {
                error!("{}", e.to_string());
                e
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

        let updated_test = self
            .test_repository
            .update_one(
                caller_id,
                test_id,
                UpdateTest {
                    submitted_at: Some(Utc::now().naive_utc()),
                    status: Some(StatusEnum::Submitted),
                    ..Default::default()
                },
            )
            .await?
            .unwrap();
        debug!("updated test {:?}", updated_test);

        let quiz_qnas = self
            .quiz_question_service
            .get_all(caller_id, test.quiz_id)
            .await?;

        // loop qua cac quiz question va lấy các đáp án đúng trong question
        // lấy selected answer của user từ quiz question
        // - Nếu là trắc nghiệm: so sánh 2 list có bằng nhau hay không
        // - Nếu là câu trả lời tự viết: todo
        let mut results = Vec::new();
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
            }
        }

        let updated_test_question_results = self
            .test_repository
            .update_test_question_results(test_id, results)
            .await?;

        Ok(updated_test_question_results)
    }

    async fn result(&self, caller_id: Uuid, test_id: Uuid) -> Result<()> {}

    async fn review_solution(
        &self,
        caller_id: Uuid,
        test_id: Uuid,
        question_id: Uuid,
    ) -> Result<()>;

    // async fn get_all(
    //     &self,
    //     caller_id: Uuid,
    //     params: QueryTestParams,
    // ) -> Result<PaginatedResponse<TestResponse>> {
    //     let PaginatedResponse {
    //         total_pages,
    //         current_page,
    //         page_size,
    //         data,
    //     } = self.test_repository.get_all(caller_id, params).await?;

    //     let mut res = Vec::<TestResponse>::new();

    //     for (test, test_state) in data.into_iter() {
    //         let status = check_test_status(test.started_at, test.submitted_at);

    //         let quiz = self.quiz_service.get_by_id(test.quiz_id).await?;
    //         let set = self.set_service.get_by_id(quiz.set_id).await?;

    //         let test_info = TestResponse {
    //             id: test.id,
    //             quiz: TestingQuiz {
    //                 id: quiz.id,
    //                 name: quiz.name,
    //                 set_id: set.id,
    //                 set_name: set.name,
    //             },
    //             status,
    //             score: test.score,
    //             created_at: test.created_at,
    //             started_at: test.started_at,
    //             submitted_at: test.submitted_at,
    //             max_duration: test.duration,
    //             remaining_time: test_state.remaining_time,
    //             current_state: None,
    //         };

    //         res.push(test_info);
    //     }

    //     Ok(PaginatedResponse {
    //         total_pages,
    //         current_page,
    //         page_size,
    //         data: res,
    //     })
    // }

    // async fn get_by_id(&self, caller_id: Uuid, test_id: Uuid) ->
    // Result<TestResponse> {     let (test, test_state) =
    // self.test_repository.get_one(test_id).await?;     let quiz =
    // self.quiz_service.get_by_id(caller_id, test.quiz_id).await?;     let set
    // = self.set_service.get_by_id(quiz.set_id).await?;

    //     let status = check_test_status(test.started_at, test.submitted_at);

    //     Ok(TestResponse {
    //         id: test.id,
    //         quiz: TestingQuiz {
    //             id: quiz.id,
    //             name: quiz.name,
    //             set_id: set.id,
    //             set_name: set.name,
    //         },
    //         status,
    //         score: test.score,
    //         created_at: test.created_at,
    //         started_at: test.started_at,
    //         submitted_at: test.submitted_at,
    //         max_duration: test.duration,
    //         remaining_time: test_state.remaining_time,
    //         current_state: Some(CurrentTestState {
    //             current_question_id: test_state.current_quiz_question,
    //             completed_questions: test_state.completed_questions,
    //             spent_time_in_second: test.duration - test_state.remaining_time,
    //         }),
    //     })
    // }

    // async fn create(&self, caller_id: Uuid, payload: CreateTest) ->
    // Result<CreateTestResponse> {     // tạo 1 đối tượng Test với quiz_id và
    // duration     let test = self
    //         .test_repository
    //         .create_one(payload.quiz_id, payload.max_duration)
    //         .await?;

    //     // lấy quiz và set để trả về cho response
    //     let quiz = self.quiz_service.get_by_id(caller_id, test.quiz_id).await?;
    //     let set = self.set_service.get_by_id(quiz.set_id).await?;

    //     Ok(CreateTestResponse {
    //         id: test.id,
    //         max_duration: test.duration,
    //         quiz: TestingQuiz {
    //             id: quiz.id,
    //             name: quiz.name,
    //             set_id: set.id,
    //             set_name: set.name,
    //         },
    //         status: "Not Started".into(),
    //         created_at: test.created_at,
    //     })
    // }

    // async fn start(&self, test_id: Uuid) -> Result<TestingQuestion> {
    //     let now = chrono::Utc::now().naive_utc();

    //     // Lấy test và test-state để làm
    //     let (test, test_state) = self.test_repository.get_one(test_id).await?;

    //     // Nếu test chưa được bắt đầu thì cập nhật trạng thái thành started
    //     if test.started_at.is_none() {
    //         // update state to started
    //         self.test_repository
    //             .update_one(
    //                 test_id,
    //                 UpdateTestParams {
    //                     started_at: Some(now),
    //                     submitted_at: None,
    //                     current_testing_quiz_question: None,
    //                     resolved_count: None,
    //                     remaining_time: None,
    //                 },
    //             )
    //             .await?;
    //     }

    //     // lấy câu hỏi quiz hiện tại trong test_state
    //     let QuizQuestionResponse {
    //         question: quiz_question,
    //         answers: quiz_question_answers,
    //     } = self
    //         .quiz_question_service
    //         .get_by_id(test_state.current_quiz_question, test.quiz_id)
    //         .await?;

    //     // lấy câu trả lời của quiz ở trên nếu có => để trả về cho FE show kết
    // quả user     // đã làm
    //     let testing_question_result = self
    //         .test_repository
    //         .get_test_question_result(test.id, quiz_question.id)
    //         .await?;
    //     let (text_answer, selected_answer_ids, spent_time_in_second) =
    //         if let Some(test_question_result) = testing_question_result {
    //             (
    //                 test_question_result.text_answer,
    //                 test_question_result.selected_answer_ids,
    //                 test_question_result.spent_time,
    //             )
    //         } else {
    //             (None, None, 0)
    //         };

    //     Ok(TestingQuestion {
    //         id: quiz_question.id,
    //         content: quiz_question.question_content,
    //         answers: quiz_question_answers
    //             .into_iter()
    //             .map(|a| a.into())
    //             .collect::<Vec<TestingAnswer>>(),
    //         r#type: quiz_question.r#type,
    //         selected_answer_ids,
    //         spent_time_in_second,
    //         text_answer,
    //         next_question_id: quiz_question.next_question,
    //         previous_question_id: quiz_question.previous_question,
    //     })
    // }

    // async fn get_test_question(&self, test_id: Uuid, question_id: Uuid) ->
    // Result<TestingQuestion> {     let (test, _) =
    // self.test_repository.get_one(test_id).await?;
    //     let QuizQuestionResponse {
    //         question: quiz_question,
    //         answers: quiz_question_answers,
    //     } = self
    //         .quiz_question_service
    //         .get_by_id(question_id, test.quiz_id)
    //         .await?;
    //     let test_question_result = self
    //         .test_repository
    //         .get_test_question_result(test.id, quiz_question.id)
    //         .await?;
    //     let (text_answer, selected_answer_ids, spent_time_in_second) =
    //         if let Some(test_question_result) = test_question_result {
    //             (
    //                 test_question_result.text_answer,
    //                 test_question_result.selected_answer_ids,
    //                 test_question_result.spent_time,
    //             )
    //         } else {
    //             (None, None, 0)
    //         };

    //     Ok(TestingQuestion {
    //         id: quiz_question.id,
    //         content: quiz_question.question_content,
    //         answers: quiz_question_answers
    //             .into_iter()
    //             .map(|a| a.into())
    //             .collect::<Vec<TestingAnswer>>(),
    //         r#type: quiz_question.r#type,
    //         selected_answer_ids,
    //         spent_time_in_second,
    //         text_answer,
    //         next_question_id: quiz_question.next_question,
    //         previous_question_id: quiz_question.previous_question,
    //     })
    // }

    // // Khi ấn next/previous/chọn câu bất kỳ/thoát => chương trình lưu lại câu trả
    // // lời vào test_result và ghi lại câu quiz đang làm vào test_state
    // async fn resolve_test_question(
    //     &self,
    //     test_id: Uuid,
    //     question_id: Uuid,
    //     payload: ResolveTestingQuestion,
    // ) -> Result<()> {
    //     let remaining_time = payload.remainning_time;

    //     // Answer: create test_question_result ->
    //     let resolved_count = self
    //         .test_repository
    //         .create_test_question_result(test_id, question_id, payload)
    //         .await?;

    //     let (test, _) = self
    //         .test_repository
    //         .update_one(
    //             test_id,
    //             UpdateTestParams {
    //                 started_at: None,
    //                 submitted_at: None,
    //                 current_testing_quiz_question: Some(question_id),
    //                 resolved_count: Some(resolved_count),
    //                 remaining_time: Some(remaining_time),
    //             },
    //         )
    //         .await?;

    //     let QuizQuestionResponse {
    //         question,
    //         answers: _,
    //     } = self
    //         .quiz_question_service
    //         .get_by_id(question_id, test.quiz_id)
    //         .await?;

    //     // TODO next

    //     Ok(())
    // }

    // // Nhấn nút submit => chương trình chấm điểm các test result
    // async fn submit(&self, test_id: Uuid) -> Result<()> {
    //     Ok(())
    // }

    // async fn result(&self, test_id: Uuid) -> Result<()> {
    //     Ok(())
    // }

    // async fn review_solution(&self, test_id: Uuid, question_id: Uuid) ->
    // Result<()> {     Ok(())
    // }
}
