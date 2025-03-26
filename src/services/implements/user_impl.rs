use {
    crate::{
        entities::users,
        enums::error::*,
        models::user::{LoginRequest, RegisterUserRequest, UpdateUserRequest, UserModel},
        repositories::user::UserRepository,
        services::traits::user_trait::UserService,
        utils::jwt::encode_jwt,
    },
    async_trait::async_trait,
    bcrypt::{hash, verify, DEFAULT_COST},
    std::sync::Arc,
    uuid::Uuid,
};

pub struct UserServiceImpl {
    user_repository: Arc<UserRepository>,
}

impl UserServiceImpl {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_all_users(&self) -> Result<Vec<users::Model>> {
        self.user_repository.get_all_users().await
    }
    async fn get_by_id(&self, user_id: Uuid) -> Result<UserModel> {
        Ok(self.user_repository.get_by_id(user_id).await?.into())
    }
    async fn get_my_info(&self, email: String) -> Result<UserModel> {
        Ok(self.user_repository.get_by_email(email).await?.into())
    }

    async fn register_user(&self, payload: RegisterUserRequest) -> Result<UserModel> {
        let RegisterUserRequest {
            email,
            password,
            name,
            role,
        } = payload;

        let hashed_password = hash(&password, DEFAULT_COST).map_err(|_| Error::HashingFailed)?;

        Ok(self
            .user_repository
            .create_user(email, hashed_password, name, role)
            .await?
            .into())
    }

    async fn login(&self, payload: LoginRequest) -> Result<String> {
        let LoginRequest { email, password } = payload;

        if let Ok(user) = self.user_repository.get_by_email(email).await {
            if !verify(password, &user.password).map_err(|_| Error::VerifyPasswordFailed)? {
                return Err(Error::LoginFailed);
            }
            encode_jwt(user.id, user.role.to_string())
        } else {
            Err(Error::LoginFailed)
        }
    }

    async fn update(
        &self,
        caller_id: Uuid,
        payload: UpdateUserRequest,
    ) -> Result<Option<UserModel>> {
        let UpdateUserRequest {
            name,
            email,
            password,
            role,
        } = payload;

        if let Some(updated_user) = self
            .user_repository
            .update_user(caller_id, name, email, password, role)
            .await?
        {
            Ok(Some(updated_user.into()))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, caller_id: Uuid) -> Result<()> {
        self.user_repository.delete_user(caller_id).await
    }
}
