use {
    crate::{
        entities::questions,
        enums::{error::*, generic::PaginatedResponse},
        models::{
            qna::{CreateQnARequest, QueryQuestionParams, UpdateQuestionRequest},
            set::SharedPermission,
        },
        repositories::question::QnARepository,
        services::traits::{prelude::SetService, qna_trait::QnAService},
    },
    async_trait::async_trait,
    std::sync::Arc,
    uuid::Uuid,
};

pub struct QnAServiceImpl {
    qna_repository: Arc<QnARepository>,
    set_service: Arc<dyn SetService>,
}

impl QnAServiceImpl {
    pub fn new(qna_repository: Arc<QnARepository>, set_service: Arc<dyn SetService>) -> Self {
        Self {
            qna_repository,
            set_service,
        }
    }
}

#[async_trait]
impl QnAService for QnAServiceImpl {
    // tạo qna trong set thì caller phải là creator của set hoặc được share set với
    // edit permission
    async fn create(&self, caller_id: Uuid, payload: CreateQnARequest) -> Result<questions::Model> {
        let is_creator_of_set = self
            .set_service
            .is_creator(payload.set_id, caller_id)
            .await?;
        let is_shared_in_edit_permission = self
            .set_service
            .check_share_permission(payload.set_id, caller_id, SharedPermission::Edit)
            .await?;

        if !is_creator_of_set && !is_shared_in_edit_permission {
            return Err(Error::PermissionDenied);
        }

        self.qna_repository.create_one(payload, caller_id).await
    }

    // Để update được thì caller phải là creator của question || creator của set
    // chứa question || được share set với edit permission
    async fn update(
        &self,
        caller_id: Uuid,
        qna_id: Uuid,
        payload: UpdateQuestionRequest,
    ) -> Result<Option<questions::Model>> {
        let set_id = self.qna_repository.get_by_id(qna_id).await?.set_id;

        // creator của question
        let is_creator_of_qna = self
            .qna_repository
            .is_creator_of_question(qna_id, caller_id)
            .await?;
        //creator của set
        let is_creator_of_set = self.set_service.is_creator(set_id, caller_id).await?;
        // được share set với edit permission
        let is_shared_in_edit_permission = self
            .set_service
            .check_share_permission(set_id, caller_id, SharedPermission::Edit)
            .await?;

        if is_creator_of_qna || is_creator_of_set || is_shared_in_edit_permission {
            self.qna_repository
                .update_question(qna_id, payload, caller_id)
                .await
        } else {
            Err(Error::PermissionDenied)
        }
    }

    // Chỉ có người tạo qna mới xoá được
    async fn delete(&self, caller_id: Uuid, qna_id: Uuid) -> Result<()> {
        if self
            .qna_repository
            .is_creator_of_question(qna_id, caller_id)
            .await?
        {
            self.qna_repository.delete_question(qna_id).await
        } else {
            Err(Error::PermissionDenied)
        }
    }

    // Lấy qna với điều kiện caller là creator của question hoặc là creator của set
    // chưa question hoặc được share hoặc set public
    async fn get_by_id(&self, caller_id: Uuid, qna_id: Uuid) -> Result<questions::Model> {
        let res = self.qna_repository.get_by_id(qna_id).await?;

        let is_existing_set = self
            .set_service
            .get_by_id(caller_id, res.set_id)
            .await
            .is_ok();

        if is_existing_set {
            return Ok(res);
        }
        Err(Error::AccessDenied)
    }

    async fn get_by_many_ids(
        &self,
        caller_id: Uuid,
        qna_ids: Vec<Uuid>,
    ) -> Result<Vec<questions::Model>> {
        let res = self.qna_repository.get_by_ids(qna_ids).await?;

        for qna in res.iter() {
            let is_existing_set = self
                .set_service
                .get_by_id(caller_id, qna.set_id)
                .await
                .is_ok();
            if !is_existing_set {
                return Err(Error::AccessDenied);
            }
        }
        return Ok(res);
    }

    async fn get_all_of_set(
        &self,
        caller_id: Uuid,
        set_id: Uuid,
        params: QueryQuestionParams,
    ) -> Result<PaginatedResponse<questions::Model>> {
        if self.set_service.get_by_id(caller_id, set_id).await.is_ok() {
            return self.qna_repository.get_all(set_id, params).await;
        }
        Err(Error::AccessDenied)
    }
}
