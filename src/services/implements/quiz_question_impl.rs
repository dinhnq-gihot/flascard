use {
    crate::{
        entities::sea_orm_active_enums::QuestionTypeEnum,
        enums::error::*,
        models::{
            quiz::{QuestionCounts, UpdateQuizRequest},
            quiz_question::{
                CreateQuizQuestionFromQuestion, CreateQuizQuestionRequest, QuizQuestionResponse,
                UpdateQuizQuestionRequest,
            },
        },
        repositories::quiz_question::QuizQuestionRepository,
        services::traits::{
            prelude::QnAService, quiz_question_trait::QuizQuestionService, quiz_trait::QuizService,
        },
        utils::{helpers::total_question_count, validator::validate_answer},
    },
    async_trait::async_trait,
    std::sync::Arc,
    uuid::Uuid,
};

pub struct QuizQuestionServiceImpl {
    quiz_question_repository: Arc<QuizQuestionRepository>,
    quiz_service: Arc<dyn QuizService>,
    qna_service: Arc<dyn QnAService>,
}

impl QuizQuestionServiceImpl {
    pub fn new(
        quiz_question_repository: Arc<QuizQuestionRepository>,
        quiz_service: Arc<dyn QuizService>,
        qna_service: Arc<dyn QnAService>,
    ) -> Self {
        Self {
            quiz_question_repository,
            quiz_service,
            qna_service,
        }
    }
}

#[async_trait]
impl QuizQuestionService for QuizQuestionServiceImpl {
    async fn create(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        payloads: Vec<CreateQuizQuestionRequest>,
    ) -> Result<Vec<QuizQuestionResponse>> {
        let quiz = self.quiz_service.get_by_id(caller_id, quiz_id).await?;
        if quiz.creator_id != caller_id {
            return Err(Error::PermissionDenied);
        }
        if quiz.is_published {
            return Err(Error::PermissionDenied);
        }

        for payload in payloads.iter() {
            if !validate_answer(&payload.r#type, &payload.answers) {
                return Err(Error::InvalidAnswer);
            }
        }

        let res = self
            .quiz_question_repository
            .create_many(quiz_id, payloads)
            .await?
            .into_iter()
            .map(|v| {
                QuizQuestionResponse {
                    question: v.0,
                    answers: v.1,
                }
            })
            .collect::<Vec<_>>();

        let mut question_counts = QuestionCounts::default();
        let mut total_point = 0;

        let all_quiz_questions = self.quiz_question_repository.get_all(quiz_id).await?;
        for (quiz_question, _) in all_quiz_questions {
            match quiz_question.r#type {
                QuestionTypeEnum::MultipleChoice => question_counts.multiple_choices += 1,
                QuestionTypeEnum::CheckBoxes => question_counts.check_boxes += 1,
                QuestionTypeEnum::TextFill => question_counts.text_fill += 1,
            }
            total_point += quiz_question.point;
        }

        self.quiz_service
            .update(
                caller_id,
                quiz_id,
                UpdateQuizRequest {
                    question_counts: Some(question_counts),
                    total_point: Some(total_point),
                    ..Default::default()
                },
            )
            .await?;

        Ok(res)
    }

    async fn create_from_question(
        &self,
        caller_id: Uuid,
        payload: CreateQuizQuestionFromQuestion,
    ) -> Result<Vec<QuizQuestionResponse>> {
        let CreateQuizQuestionFromQuestion {
            quiz_id,
            question_ids,
        } = payload;

        let quiz = self.quiz_service.get_by_id(caller_id, quiz_id).await?;
        if quiz.creator_id != caller_id {
            return Err(Error::PermissionDenied);
        }
        if quiz.is_published {
            return Err(Error::PermissionDenied);
        }

        let question_counts = serde_json::from_value::<QuestionCounts>(quiz.question_counts)
            .map_err(|e| Error::Anyhow(e.into()))?;
        let mut quiz_question_counts_number = total_question_count(question_counts);

        let creating_questions = self
            .qna_service
            .get_by_many_ids(caller_id, question_ids)
            .await?
            .into_iter()
            .map(|question| {
                let mut creating_quiz_question: CreateQuizQuestionRequest = question.into();
                creating_quiz_question.index = quiz_question_counts_number;
                quiz_question_counts_number += 1;
                creating_quiz_question
            })
            .collect::<Vec<CreateQuizQuestionRequest>>();

        let res = self
            .quiz_question_repository
            .create_many(quiz_id, creating_questions)
            .await?
            .into_iter()
            .map(|r| {
                QuizQuestionResponse {
                    question: r.0,
                    answers: r.1,
                }
            })
            .collect::<Vec<_>>();

        Ok(res)
    }

    async fn update(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        payloads: Vec<UpdateQuizQuestionRequest>,
    ) -> Result<Vec<QuizQuestionResponse>> {
        // Need to validate answers

        let quiz = self.quiz_service.get_by_id(caller_id, quiz_id).await?;
        if quiz.creator_id != caller_id {
            return Err(Error::PermissionDenied);
        }
        if quiz.is_published {
            return Err(Error::PermissionDenied);
        }

        let res = self
            .quiz_question_repository
            .update_many(payloads)
            .await?
            .into_iter()
            .map(|v| {
                QuizQuestionResponse {
                    question: v.0,
                    answers: v.1,
                }
            })
            .collect::<Vec<_>>();

        let mut question_counts = QuestionCounts::default();
        let mut total_point = 0;

        let all_quiz_questions = self.quiz_question_repository.get_all(quiz_id).await?;
        for (quiz_question, _) in all_quiz_questions {
            match quiz_question.r#type {
                QuestionTypeEnum::MultipleChoice => question_counts.multiple_choices += 1,
                QuestionTypeEnum::CheckBoxes => question_counts.check_boxes += 1,
                QuestionTypeEnum::TextFill => question_counts.text_fill += 1,
            }
            total_point += quiz_question.point;
        }

        self.quiz_service
            .update(
                caller_id,
                quiz_id,
                UpdateQuizRequest {
                    question_counts: Some(question_counts),
                    total_point: Some(total_point),
                    ..Default::default()
                },
            )
            .await?;

        Ok(res)
    }

    async fn delete(&self, caller_id: Uuid, quiz_id: Uuid, quiz_question_id: Uuid) -> Result<()> {
        if !self.quiz_service.is_created_by(quiz_id, caller_id).await? {
            return Err(Error::PermissionDenied);
        }

        self.quiz_question_repository
            .delete(quiz_question_id, quiz_id)
            .await
    }

    async fn get_by_id(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        quiz_question_id: Uuid,
    ) -> Result<QuizQuestionResponse> {
        if !self.quiz_service.is_created_by(quiz_id, caller_id).await?
            || !self.quiz_service.is_shared_with(quiz_id, caller_id).await?
        {
            return Err(Error::PermissionDenied);
        }

        let (question, answers) = self
            .quiz_question_repository
            .get_by_id(quiz_question_id, quiz_id)
            .await?;

        Ok(QuizQuestionResponse { question, answers })
    }

    async fn get_by_index(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        quiz_question_index: i32,
    ) -> Result<QuizQuestionResponse> {
        if !self.quiz_service.is_created_by(quiz_id, caller_id).await?
            || !self.quiz_service.is_shared_with(quiz_id, caller_id).await?
        {
            return Err(Error::PermissionDenied);
        }

        let (question, answers) = self
            .quiz_question_repository
            .get_by_index(quiz_question_index, quiz_id)
            .await?;

        Ok(QuizQuestionResponse { question, answers })
    }

    async fn get_all(&self, caller_id: Uuid, quiz_id: Uuid) -> Result<Vec<QuizQuestionResponse>> {
        if !self.quiz_service.is_created_by(quiz_id, caller_id).await?
            || !self.quiz_service.is_shared_with(quiz_id, caller_id).await?
        {
            return Err(Error::PermissionDenied);
        }

        Ok(self
            .quiz_question_repository
            .get_all(quiz_id)
            .await?
            .into_iter()
            .map(|q| {
                QuizQuestionResponse {
                    question: q.0,
                    answers: q.1,
                }
            })
            .collect::<Vec<_>>())
    }
}
