use {
    crate::{
        entities::{sets, shared_sets},
        enums::error::*,
        models::set::{AllSetsOfUserResponse, CreateSetRequest, ShareSetForUser, UpdateSetRequest},
        repositories::set::SetRepository,
        services::traits::set_trait::SetService,
    },
    async_trait::async_trait,
    std::sync::Arc,
    uuid::Uuid,
};

pub struct SetServiceImpl {
    set_repository: Arc<SetRepository>,
}

impl SetServiceImpl {
    pub fn new(set_repository: Arc<SetRepository>) -> Self {
        Self { set_repository }
    }
}

#[async_trait]
impl SetService for SetServiceImpl {
    async fn create(&self, caller_id: Uuid, payload: CreateSetRequest) -> Result<sets::Model> {
        let CreateSetRequest {
            name,
            description,
            public_or_not,
        } = payload;

        self.set_repository
            .create_one(caller_id, name, description, public_or_not)
            .await
    }

    async fn get_by_id(&self, id: Uuid) -> Result<sets::Model> {
        self.set_repository.get_by_id(id).await
    }

    async fn get_all(&self) -> Result<Vec<sets::Model>> {
        self.set_repository.get_all_set().await
    }

    async fn update(
        &self,
        caller_id: Uuid,
        set_id: Uuid,
        payload: UpdateSetRequest,
    ) -> Result<Option<sets::Model>> {
        self.set_repository.is_owner(set_id, caller_id).await?;

        let UpdateSetRequest {
            name,
            description,
            public_or_not,
        } = payload;

        self.set_repository
            .update_one(set_id, name, description, public_or_not)
            .await
    }

    async fn delete(&self, caller_id: Uuid, set_id: Uuid) -> Result<()> {
        self.set_repository.is_owner(set_id, caller_id).await?;
        self.set_repository.delete_one(set_id).await
    }

    async fn get_all_sets_of_user(&self, user_id: Uuid) -> Result<AllSetsOfUserResponse> {
        let owned_sets = self.set_repository.get_by_owner_id(user_id).await?;
        let shared_sets = self
            .set_repository
            .get_all_shared_sets_of_user(user_id)
            .await?;
        Ok(AllSetsOfUserResponse {
            owned_sets,
            shared_sets,
        })
    }

    async fn share(
        &self,
        caller_id: Uuid,
        set_id: Uuid,
        payload: Vec<ShareSetForUser>,
    ) -> Result<Vec<shared_sets::Model>> {
        if self
            .set_repository
            .is_owner(set_id, caller_id)
            .await
            .is_ok()
        {
            return self.set_repository.create_share_set(set_id, payload).await;
        }

        Err(Error::PermissionDenied)
    }
}
