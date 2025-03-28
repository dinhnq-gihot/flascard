use {
    crate::{
        entities::{sea_orm_active_enums::RoleEnum, users},
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
    // Done ✅
    async fn get_all_users(&self) -> Result<Vec<users::Model>> {
        self.user_repository.get_all_users().await
    }

    // Done ✅
    async fn get_by_id(&self, user_id: Uuid) -> Result<UserModel> {
        Ok(self.user_repository.get_by_id(user_id).await?.into())
    }

    // Done ✅
    async fn get_my_info(&self, email: String) -> Result<UserModel> {
        Ok(self.user_repository.get_by_email(email).await?.into())
    }

    // Done ✅
    async fn register_user(&self, mut payload: RegisterUserRequest) -> Result<UserModel> {
        let hashed_password =
            hash(&payload.password, DEFAULT_COST).map_err(|_| Error::HashingFailed)?;

        payload.password = hashed_password;

        Ok(self.user_repository.create_user(payload).await?.into())
    }

    // Done ✅
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

    // Done ✅
    async fn update_self(
        &self,
        caller_id: Uuid,
        payload: UpdateUserRequest,
    ) -> Result<Option<UserModel>> {
        if let Some(updated_user) = self.user_repository.update_user(caller_id, payload).await? {
            Ok(Some(updated_user.into()))
        } else {
            Ok(None)
        }
    }

    // Done ✅
    async fn update_password(
        &self,
        caller_id: Uuid,
        old_password: String,
        new_password: String,
    ) -> Result<Option<UserModel>> {
        if let Ok(user) = self.user_repository.get_by_id(caller_id).await {
            if !verify(old_password, &user.password).map_err(|_| Error::VerifyPasswordFailed)? {
                return Err(Error::VerifyPasswordFailed);
            }

            let hashed_password =
                hash(&new_password, DEFAULT_COST).map_err(|_| Error::HashingFailed)?;

            Ok(self
                .user_repository
                .update_user(
                    caller_id,
                    UpdateUserRequest {
                        password: Some(hashed_password),
                        ..Default::default()
                    },
                )
                .await?
                .map(Into::into))
        } else {
            Err(Error::RecordNotFound)
        }
    }

    // only admin role
    // Done ✅
    async fn update_role(
        &self,
        caller_id: Uuid,
        user_id: Uuid,
        role: RoleEnum,
    ) -> Result<Option<UserModel>> {
        let is_staff = self
            .user_repository
            .check_role(caller_id, RoleEnum::Staff)
            .await?;
        if is_staff {
            Ok(self
                .user_repository
                .update_user(
                    user_id,
                    UpdateUserRequest {
                        role: Some(role),
                        ..Default::default()
                    },
                )
                .await?
                .map(Into::into))
        } else {
            Err(Error::PermissionDenied)
        }
    }

    // Done ✅
    async fn delete(&self, caller_id: Uuid) -> Result<()> {
        self.user_repository.delete_user(caller_id).await
    }
}
