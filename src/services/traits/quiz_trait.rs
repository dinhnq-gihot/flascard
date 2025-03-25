use {
    crate::{
        entities::{quizes, shared_quizes, users},
        enums::error::*,
        models::quiz::{CreateQuizRequest, FilterQuizParams, UpdateQuizRequest},
    },
    async_trait::async_trait,
    uuid::Uuid,
};

#[async_trait]
pub trait QuizService: Send + Sync {
    async fn create_one(
        &self,
        creator_id: Uuid,
        payload: CreateQuizRequest,
    ) -> Result<quizes::Model>;
    async fn update_one(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        payload: UpdateQuizRequest,
    ) -> Result<Option<quizes::Model>>;
    async fn delete_one(&self, quiz_id: Uuid, caller: Uuid) -> Result<()>;
    async fn get_by_id(&self, quiz_id: Uuid) -> Result<quizes::Model>;
    async fn get_all(&self, params: FilterQuizParams) -> Result<Vec<quizes::Model>>;

    // SHARE SESSION
    async fn share(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        new_participants: Vec<Uuid>,
    ) -> Result<Vec<shared_quizes::Model>>;

    async fn get_all_shared_quizzes_of_user(&self, user_id: Uuid) -> Result<Vec<quizes::Model>>;

    async fn get_all_shared_users_of_quiz(&self, quiz_id: Uuid) -> Result<Vec<users::Model>>;

    async fn is_created_by(&self, quiz_id: Uuid, user_id: Uuid) -> Result<bool>;

    async fn is_shared_with(&self, quiz_id: Uuid, user_id: Uuid) -> Result<bool>;
}
