use {
    crate::{
        entities::{quizes, shared_quizes},
        enums::{error::*, generic::PaginatedResponse},
        models::{
            quiz::{CreateQuizRequest, FilterQuizParams, QuizWithVisibility, UpdateQuizRequest},
            user::UserModel,
        },
        repositories::quiz::QuizRepository,
        services::traits::quiz_trait::QuizService,
    },
    async_trait::async_trait,
    std::sync::Arc,
    uuid::Uuid,
};

pub struct QuizServiceImpl {
    quiz_repository: Arc<QuizRepository>,
}

impl QuizServiceImpl {
    pub fn new(quiz_repository: Arc<QuizRepository>) -> Self {
        Self { quiz_repository }
    }
}

#[async_trait]
impl QuizService for QuizServiceImpl {
    async fn create(&self, creator_id: Uuid, payload: CreateQuizRequest) -> Result<quizes::Model> {
        self.quiz_repository.create_one(payload, creator_id).await
    }

    async fn update(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        payload: UpdateQuizRequest,
    ) -> Result<Option<quizes::Model>> {
        let quiz = self.quiz_repository.get_by_id(caller_id, quiz_id).await?;
        if caller_id != quiz.creator_id {
            return Err(Error::PermissionDenied);
        }
        if quiz.is_published {
            return Err(Error::Published);
        }

        self.quiz_repository.update_one(quiz_id, payload).await
    }

    async fn delete(&self, caller_id: Uuid, quiz_id: Uuid) -> Result<()> {
        if self.is_created_by(quiz_id, caller_id).await? {
            return self.quiz_repository.delete_one(quiz_id).await;
        }
        Err(Error::PermissionDenied)
    }

    async fn get_by_id(&self, caller_id: Uuid, quiz_id: Uuid) -> Result<quizes::Model> {
        self.quiz_repository.get_by_id(caller_id, quiz_id).await
    }

    async fn get_all(
        &self,
        caller_id: Uuid,
        params: FilterQuizParams,
    ) -> Result<PaginatedResponse<QuizWithVisibility>> {
        self.quiz_repository.get_all(caller_id, params).await
    }

    async fn share(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        new_participants: Vec<Uuid>,
    ) -> Result<Vec<shared_quizes::Model>> {
        let quiz = self.quiz_repository.get_by_id(caller_id, quiz_id).await?;
        if caller_id != quiz.creator_id {
            return Err(Error::AccessDenied);
        }

        self.quiz_repository
            .create_share(quiz_id, new_participants)
            .await
    }

    async fn get_all_shared_users_of_quiz(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<Vec<UserModel>> {
        if self
            .quiz_repository
            .is_created_by(quiz_id, caller_id)
            .await?
        {
            return self
                .quiz_repository
                .get_all_shared_users_of_quiz(quiz_id)
                .await;
        }
        Err(Error::PermissionDenied)
    }

    async fn is_created_by(&self, quiz_id: Uuid, user_id: Uuid) -> Result<bool> {
        self.quiz_repository.is_created_by(quiz_id, user_id).await
    }

    async fn is_shared_with(&self, quiz_id: Uuid, user_id: Uuid) -> Result<bool> {
        self.quiz_repository.is_shared_with(quiz_id, user_id).await
    }
}
