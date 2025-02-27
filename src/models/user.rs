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

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: RoleEnum,
}

impl From<users::Model> for UserResponse {
    fn from(value: users::Model) -> Self {
        Self {
            id: value.id,
            email: value.email,
            name: value.name,
            role: value.role,
        }
    }
}
