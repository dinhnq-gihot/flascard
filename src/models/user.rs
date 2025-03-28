use {
    crate::entities::{sea_orm_active_enums::RoleEnum, users},
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    pub role: RoleEnum,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct UpdateUserRequest {
    // for update info self
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,

    pub password: Option<String>,
    // only staff update role
    pub role: Option<RoleEnum>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPassword {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRole {
    pub user_id: Uuid,
    pub new_role: RoleEnum,
}

#[derive(Debug, Deserialize)]
pub struct DeleteRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: RoleEnum,
    pub avatar_url: Option<String>,
}

impl From<users::Model> for UserModel {
    fn from(value: users::Model) -> Self {
        Self {
            id: value.id,
            email: value.email,
            name: value.name,
            role: value.role,
            avatar_url: value.avatar_url,
        }
    }
}
