use {
    crate::entities::{sea_orm_active_enums::PermissionEnum, sets},
    sea_orm::FromQueryResult,
    serde::{Deserialize, Serialize},
    serde_json::Value as JsonValue,
    uuid::Uuid,
};

#[derive(Debug, Deserialize)]
pub struct CreateSetRequest {
    pub name: String,
    pub description: Option<String>,
    pub public_or_not: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSetRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub public_or_not: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ShareSetForUser {
    pub user_id: Uuid,
    pub permission: Option<PermissionEnum>,
}

#[derive(Debug, Serialize, FromQueryResult)]
pub struct SharedSetsWithPermission {
    pub set: JsonValue,
    pub permission: PermissionEnum,
}

#[derive(Debug, Serialize)]
pub struct AllSetsOfUserResponse {
    pub own_sets: Vec<sets::Model>,
    pub shared_sets: Vec<SharedSetsWithPermission>,
    pub public_sets: Vec<sets::Model>,
}

// pub struct AllUsersOfSetResponse {
//     pub
// }
