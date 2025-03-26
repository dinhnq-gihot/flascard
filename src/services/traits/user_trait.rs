use {
    crate::{
        entities::users,
        enums::error::*,
        models::user::{LoginRequest, RegisterUserRequest, UpdateUserRequest, UserModel},
    },
    async_trait::async_trait,
    uuid::Uuid,
};

#[async_trait]
pub trait UserService {
    async fn get_all_users(&self) -> Result<Vec<users::Model>>;
    async fn get_by_id(&self, user_id: Uuid) -> Result<UserModel>;
    async fn get_my_info(&self, email: String) -> Result<UserModel>;

    async fn register_user(&self, payload: RegisterUserRequest) -> Result<UserModel>;

    async fn login(&self, payload: LoginRequest) -> Result<String>;

    async fn update(
        &self,
        caller_id: Uuid,
        payload: UpdateUserRequest,
    ) -> Result<Option<UserModel>>;

    async fn delete(&self, user_id: Uuid) -> Result<()>;
}
