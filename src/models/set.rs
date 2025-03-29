use {
    crate::entities::sets,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
pub enum SharedPermission {
    #[default]
    View = 0,
    Comment = 1,
    Edit = 2,
}

impl From<i32> for SharedPermission {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::View,
            1 => Self::Comment,
            2 => Self::Edit,
            _ => Self::View,
        }
    }
}

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
    pub permission: Option<SharedPermission>,
}

#[derive(Debug, Serialize)]
pub struct SharedSetsWithPermission {
    pub set: sets::Model,
    pub permission: SharedPermission,
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
