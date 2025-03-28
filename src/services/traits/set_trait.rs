use {
    crate::{
        entities::{sets, shared_sets},
        enums::error::*,
        models::set::{AllSetsOfUserResponse, CreateSetRequest, ShareSetForUser, UpdateSetRequest},
    },
    async_trait::async_trait,
    uuid::Uuid,
};

// Set chỉ lấy được khi đăng nhập vào => vì set là ngân hàng đề => set nhằm mục
// đích để có người lấy để tạo quiz
#[async_trait]
pub trait SetService: Send + Sync {
    // Create a set
    async fn create(&self, caller_id: Uuid, payload: CreateSetRequest) -> Result<sets::Model>;

    // Upload a document file to create set of flashcards (AI feature)
    // async fn upload(&self, caller_id: Uuid) -> Result<sets::Model>;

    // Update information of a set
    async fn update(
        &self,
        caller_id: Uuid,
        set_id: Uuid,
        payload: UpdateSetRequest,
    ) -> Result<Option<sets::Model>>;

    //Delete a set
    async fn delete(&self, caller_id: Uuid, set_id: Uuid) -> Result<()>;

    // Get all sets includes created by user or shared with or public
    async fn get_all(&self, caller_id: Uuid) -> Result<AllSetsOfUserResponse>;

    // Get one set includes created by user or shared with or public
    async fn get_by_id(&self, caller_id: Uuid, set_id: Uuid) -> Result<sets::Model>;

    // Share a set with other users or public
    async fn share(
        &self,
        caller_id: Uuid,
        set_id: Uuid,
        payload: Vec<ShareSetForUser>,
    ) -> Result<Vec<shared_sets::Model>>;
}
