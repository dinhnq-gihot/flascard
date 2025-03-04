use {
    crate::{
        db::db_connection::Database,
        entities::{
            prelude::{Sets, Users},
            sea_orm_active_enums::PermissionEnum,
            sets, shared_sets, users,
        },
        enums::error::*,
    },
    sea_orm::{
        ActiveModelTrait, ColumnTrait, EntityTrait, JoinType, ModelTrait, QueryFilter, QuerySelect,
        RelationTrait, Set,
    },
    std::sync::Arc,
    uuid::Uuid,
};

pub struct SharedSetService {
    db: Arc<Database>,
}

impl SharedSetService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn create_shared_set(
        &self,
        user_id: Uuid,
        set_id: Uuid,
        permission: Option<PermissionEnum>,
    ) -> Result<shared_sets::Model> {
        let conn = self.db.get_connection().await;
        let mut shared_set = shared_sets::ActiveModel {
            set_id: Set(set_id),
            user_id: Set(user_id),
            ..Default::default()
        };
        if let Some(p) = permission {
            shared_set.permission = Set(p);
        }

        shared_set.insert(&conn).await.map_err(Error::InsertFailed)
    }

    pub async fn get_shared_set(&self, set_id: Uuid, user_id: Uuid) -> Result<Option<sets::Model>> {
        let conn = self.db.get_connection().await;

        Sets::find()
            .join(JoinType::InnerJoin, sets::Relation::SharedSets.def())
            .filter(shared_sets::Column::UserId.eq(user_id))
            .filter(shared_sets::Column::UserId.eq(set_id))
            .filter(sets::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)
    }

    pub async fn get_all_shared_users_of_set(&self, set_id: Uuid) -> Result<Vec<users::Model>> {
        let conn = self.db.get_connection().await;

        Users::find()
            .join(JoinType::InnerJoin, users::Relation::SharedSets.def())
            .filter(shared_sets::Column::SetId.eq(set_id))
            .filter(users::Column::IsDeleted.eq(false))
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)
    }

    pub async fn get_all_shared_sets_of_user(&self, user_id: Uuid) -> Result<Vec<sets::Model>> {
        let conn = self.db.get_connection().await;

        let user = Users::find_by_id(user_id)
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?;

        user.find_related(Sets)
            .filter(sets::Column::IsDeleted.eq(false))
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)

        // Sets::find()
        //     .join(JoinType::InnerJoin, sets::Relation::SharedSets.def())
        //     .filter(shared_sets::Column::UserId.eq(user_id))
        //     .filter(sets::Column::IsDeleted.eq(false))
        //     .all(&conn)
        //     .await
        //     .map_err(Error::QueryFailed)
    }
}
