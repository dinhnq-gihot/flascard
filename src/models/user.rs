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
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<RoleEnum>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteRequest {
    pub id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: RoleEnum,
}

impl From<users::Model> for UserModel {
    fn from(value: users::Model) -> Self {
        Self {
            id: value.id,
            email: value.email,
            name: value.name,
            role: value.role,
        }
    }
}
