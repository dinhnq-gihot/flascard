use {crate::{entities::sets, enums::error::*}, async_trait::async_trait, uuid::Uuid};

#[async_trait]
pub trait SetService {
    async fn get_all(&self) -> Result<Vec<sets::Model>>;
    async fn get_by_id(&self, id: Uuid) -> Result<sets::Model>;
    async fn get_by_owner_id(&self, owner_id: Uuid) -> Result<Vec<sets::Model>>;
}
