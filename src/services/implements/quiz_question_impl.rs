use {
    crate::{
        entities::{quiz_question_answers, quiz_questions},
        enums::error::*,
        models::{
            quiz::UpdateQuizRequest,
            quiz_question::{
                CreateQuizQuestionRequest, QuizQuestionResponse, UpdateQuizQuestionRequest,
            },
        },
        repositories::quiz_question::QuizQuestionRepository,
        services::traits::{quiz_question_trait::QuizQuestionService, quiz_trait::QuizService},
        utils::validator::{all_quiz_answers_contain_id, validate_answer},
    },
    async_trait::async_trait,
    std::sync::Arc,
    uuid::Uuid,
};

pub struct QuizQuestionServiceImpl {
    quiz_question_repository: Arc<QuizQuestionRepository>,
    quiz_service: Arc<dyn QuizService>,
}

impl QuizQuestionServiceImpl {
    pub fn new(
        quiz_question_repository: Arc<QuizQuestionRepository>,
        quiz_service: Arc<dyn QuizService>,
    ) -> Self {
        Self {
            quiz_question_repository,
            quiz_service,
        }
    }
}

#[async_trait]
impl QuizQuestionService for QuizQuestionServiceImpl {
    async fn create_one(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        payload: CreateQuizQuestionRequest,
    ) -> Result<QuizQuestionResponse> {
        if !validate_answer(&payload.r#type, &payload.answers) {
            return Err(Error::InvalidAnswer);
        }
        let quiz = self.quiz_service.get_by_id(caller_id, quiz_id).await?;

        if quiz.creator_id != caller_id {
            return Err(Error::PermissionDenied);
        }
        if quiz.is_published {
            return Err(Error::PermissionDenied);
        }

        let new_question = self
            .quiz_question_repository
            .create_one(quiz.id, quiz.last_question, payload.clone())
            .await?;
        if let Some(last_question_id) = quiz.last_question {
            self.quiz_question_repository
                .update_one(
                    last_question_id,
                    quiz.id,
                    UpdateQuizQuestionRequest {
                        question_content: None,
                        answers: None,
                        previous_question_id: None,
                        next_question_id: Some(new_question.id),
                    },
                )
                .await?;
        }
        let new_answers = self
            .quiz_question_repository
            .create_answers(new_question.id, payload.answers)
            .await?;

        self.quiz_service
            .update_one(
                caller_id,
                quiz.id,
                UpdateQuizRequest {
                    is_public: None,
                    publish: None,
                    last_question_id: Some(new_question.id),
                },
            )
            .await?;
        Ok(QuizQuestionResponse {
            question: new_question,
            answers: new_answers,
        })
    }

    async fn update_one(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        quiz_question_id: Uuid,
        payload: UpdateQuizQuestionRequest,
    ) -> Result<Option<QuizQuestionResponse>> {
        let quiz = self.quiz_service.get_by_id(caller_id, quiz_id).await?;
        let (quiz_question, _) = self
            .quiz_question_repository
            .get_by_id(quiz_question_id, quiz_id)
            .await?;

        if quiz.creator_id != caller_id {
            return Err(Error::PermissionDenied);
        }
        if quiz.is_published {
            return Err(Error::PermissionDenied);
        }
        if let Some(answers) = &payload.answers {
            // Kiểm tra xem answer đc chỉnh có phù hợp không
            // và tất cả các answer phải chưa id để update trong db
            if !validate_answer(&quiz_question.r#type, answers)
                || !all_quiz_answers_contain_id(answers)
            {
                return Err(Error::InvalidAnswer);
            }
            let answer_ids = answers.iter().map(|a| a.id.unwrap()).collect::<Vec<Uuid>>();

            // Kiểm tra các answer có cùng 1 question không
            if !self
                .quiz_question_repository
                .is_of_question(quiz_question_id, answer_ids)
                .await?
            {
                return Err(Error::InvalidAnswer);
            }
        }

        if let Some((updated_question, updated_answers)) = self
            .quiz_question_repository
            .update_one(quiz_question_id, quiz_id, payload)
            .await?
        {
            Ok(Some(QuizQuestionResponse {
                question: updated_question,
                answers: updated_answers,
            }))
        } else {
            Ok(None)
        }
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

    async fn get_all(&self, caller_id: Uuid, quiz_id: Uuid) -> Result<Vec<quiz_questions::Model>> {
        if !self.quiz_service.is_created_by(quiz_id, caller_id).await?
            || !self.quiz_service.is_shared_with(quiz_id, caller_id).await?
        {
            return Err(Error::PermissionDenied);
        }

        self.quiz_question_repository.get_all(quiz_id).await
    }
}
