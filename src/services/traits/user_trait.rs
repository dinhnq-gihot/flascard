use {
    crate::{
        entities::{sea_orm_active_enums::RoleEnum, users},
        enums::error::*,
        models::user::{LoginRequest, RegisterUserRequest, UpdateUserRequest, UserModel},
    },
    async_trait::async_trait,
    uuid::Uuid,
};

#[async_trait]
pub trait UserService: Sync + Send {
    async fn get_all_users(&self) -> Result<Vec<users::Model>>;
    async fn get_by_id(&self, user_id: Uuid) -> Result<UserModel>;
    async fn get_my_info(&self, email: String) -> Result<UserModel>;

    async fn register_user(&self, payload: RegisterUserRequest) -> Result<UserModel>;

    async fn login(&self, payload: LoginRequest) -> Result<String>;

    async fn update_self(
        &self,
        caller_id: Uuid,
        payload: UpdateUserRequest,
    ) -> Result<Option<UserModel>>;

    async fn update_password(
        &self,
        caller_id: Uuid,
        old_password: String,
        new_password: String,
    ) -> Result<Option<UserModel>>;

    // only admin role
    async fn update_role(
        &self,
        caller_id: Uuid,
        user_id: Uuid,
        role: RoleEnum,
    ) -> Result<Option<UserModel>>;

    async fn delete(&self, user_id: Uuid) -> Result<()>;

    // forget password
}
