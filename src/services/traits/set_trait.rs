use {
    crate::{
        entities::{sets, shared_sets},
        enums::error::*,
        models::set::{AllSetsOfUserResponse, CreateSetRequest, ShareSetForUser, UpdateSetRequest},
    },
    async_trait::async_trait,
    uuid::Uuid,
};

#[async_trait]
pub trait SetService: Send + Sync {
    async fn get_all(&self) -> Result<Vec<sets::Model>>;
    async fn get_by_id(&self, id: Uuid) -> Result<sets::Model>;
    async fn create(&self, caller_id: Uuid, payload: CreateSetRequest) -> Result<sets::Model>;
    async fn update(
        &self,
        caller_id: Uuid,
        set_id: Uuid,
        payload: UpdateSetRequest,
    ) -> Result<Option<sets::Model>>;
    async fn delete(&self, caller_id: Uuid, set_id: Uuid) -> Result<()>;
    async fn get_all_sets_of_user(&self, user_id: Uuid) -> Result<AllSetsOfUserResponse>;
    async fn share(
        &self,
        caller_id: Uuid,
        set_id: Uuid,
        payload: Vec<ShareSetForUser>,
    ) -> Result<Vec<shared_sets::Model>>;
}
