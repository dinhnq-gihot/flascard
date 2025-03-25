use {
    crate::{
        entities::questions,
        enums::{error::*, generic::PaginatedResponse},
        models::qna::{CreateQnARequest, QueryQuestionParams, UpdateQuestionRequest},
        repositories::question::QnARepository,
        services::traits::qna_trait::QnAService,
    },
    async_trait::async_trait,
    std::sync::Arc,
    uuid::Uuid,
};

pub struct QnAServiceImpl {
    qna_repository: Arc<QnARepository>,
}

impl QnAServiceImpl {
    pub fn new(qna_repository: Arc<QnARepository>) -> Self {
        Self { qna_repository }
    }
}

#[async_trait]
impl QnAService for QnAServiceImpl {
    async fn create(&self, caller_id: Uuid, payload: CreateQnARequest) -> Result<questions::Model> {
        self.qna_repository.create_one(payload, caller_id).await
    }

    async fn update(
        &self,
        caller_id: Uuid,
        qna_id: Uuid,
        payload: UpdateQuestionRequest,
    ) -> Result<Option<questions::Model>> {
        if self
            .qna_repository
            .is_creator_of_question(qna_id, caller_id)
            .await?
        {
            self.qna_repository.update_question(qna_id, payload).await
        } else {
            Err(Error::PermissionDenied)
        }
    }

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

    async fn get_by_id(&self, qna_id: Uuid) -> Result<questions::Model> {
        self.qna_repository.get_by_id(qna_id).await
    }

    async fn get_all(
        &self,
        params: QueryQuestionParams,
    ) -> Result<PaginatedResponse<questions::Model>> {
        self.qna_repository.get_all(params).await
    }
}
