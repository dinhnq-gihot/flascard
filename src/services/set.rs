use {
    crate::{
        db::db::Database,
        entities::{
            prelude::{Sets, SharedSets},
            sea_orm_active_enums::PermissionEnum,
            sets, shared_sets,
        },
        enums::error::*,
    },
    sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set},
    std::sync::Arc,
    uuid::Uuid,
};

pub struct SetService {
    db: Arc<Database>,
}

impl SetService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_all_set(&self) -> Result<Vec<sets::Model>> {
        let conn = self.db.get_connection().await;
        Sets::find().all(&conn).await.map_err(Error::QueryFailed)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<sets::Model> {
        let conn = self.db.get_connection().await;
        Sets::find_by_id(id)
            .filter(sets::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)
    }

    pub async fn get_by_owner_id(&self, owner_id: Uuid) -> Result<Vec<sets::Model>> {
        let conn = self.db.get_connection().await;
        Sets::find()
            .filter(sets::Column::OwnerId.eq(owner_id))
            .filter(sets::Column::IsDeleted.eq(false))
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)
    }

    pub async fn create_set(
        &self,
        creator_id: Uuid,
        name: String,
        description: Option<String>,
        public_or_not: Option<bool>,
    ) -> Result<sets::Model> {
        let conn = self.db.get_connection().await;

        let mut new_set = sets::ActiveModel {
            name: Set(name),
            owner_id: Set(creator_id),
            description: Set(description),
            ..Default::default()
        };
        if let Some(p) = public_or_not {
            new_set.public_or_not = Set(p);
        };

        Sets::insert(new_set)
            .exec_with_returning(&conn)
            .await
            .map_err(Error::InsertFailed)
    }

    pub async fn update_set(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        public_or_not: Option<bool>,
    ) -> Result<sets::Model> {
        let conn = self.db.get_connection().await;

        let mut set: sets::ActiveModel = Sets::find_by_id(id)
            .filter(sets::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?
            .into();

        if let Some(name) = name {
            set.name = Set(name);
        }
        if let Some(d) = description {
            set.description = Set(Some(d));
        }
        if let Some(p) = public_or_not {
            set.public_or_not = Set(p);
        }

        set.update(&conn).await.map_err(Error::UpdateFailed)
    }

    pub async fn delete_set(&self, id: Uuid) -> Result<()> {
        let conn = self.db.get_connection().await;
        let mut set: sets::ActiveModel = Sets::find_by_id(id)
            .filter(sets::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?
            .into();

        set.is_deleted = Set(true);
        let _ = set.update(&conn).await.map_err(Error::UpdateFailed)?;

        Ok(())
    }

    pub async fn check_permission(&self, id: Uuid, user_id: Uuid) -> Result<()> {
        let conn = self.db.get_connection().await;
        // check owner permission first
        let set = Sets::find_by_id(id)
            .filter(sets::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?;

        if set.owner_id != user_id {
            // check edit permission later
            let shared_set = SharedSets::find()
                .filter(shared_sets::Column::SetId.eq(id))
                .filter(shared_sets::Column::UserId.eq(user_id))
                .one(&conn)
                .await
                .map_err(Error::QueryFailed)?
                .ok_or(Error::PermissionDenied)?;
            if shared_set.permission != PermissionEnum::Edit {
                return Err(Error::PermissionDenied);
            }
        }

        Ok(())
    }
}
