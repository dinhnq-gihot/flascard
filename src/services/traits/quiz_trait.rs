use {
    crate::{
        entities::{quizes, shared_quizes},
        enums::{error::*, generic::PaginatedResponse},
        models::{
            quiz::{CreateQuizRequest, FilterQuizParams, QuizWithVisibility, UpdateQuizRequest},
            user::UserModel,
        },
    },
    async_trait::async_trait,
    uuid::Uuid,
};

#[async_trait]
pub trait QuizService: Send + Sync {
    // tạo 1 quiz
    async fn create(&self, creator_id: Uuid, payload: CreateQuizRequest) -> Result<quizes::Model>;

    // cập nhật metadata của quiz
    async fn update(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        payload: UpdateQuizRequest,
    ) -> Result<Option<quizes::Model>>;

    async fn delete(&self, caller_id: Uuid, quiz_id: Uuid) -> Result<()>;

    async fn get_by_id(&self, caller_id: Uuid, quiz_id: Uuid) -> Result<quizes::Model>;

    // lấy toàn bộ quiz mà user tạo/được share/public
    async fn get_all(
        &self,
        caller_id: Uuid,
        params: FilterQuizParams,
    ) -> Result<PaginatedResponse<QuizWithVisibility>>;

    async fn is_created_by(&self, quiz_id: Uuid, user_id: Uuid) -> Result<bool>;

    // SHARE SESSION
    async fn share(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
        new_participants: Vec<Uuid>,
    ) -> Result<Vec<shared_quizes::Model>>;

    async fn get_all_shared_users_of_quiz(
        &self,
        caller_id: Uuid,
        quiz_id: Uuid,
    ) -> Result<Vec<UserModel>>;

    async fn is_shared_with(&self, quiz_id: Uuid, user_id: Uuid) -> Result<bool>;
}
