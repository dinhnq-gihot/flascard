use {
    crate::{
        db::db_connection::Database,
        debug,
        entities::{
            prelude::{Sets, SharedSets, Users},
            sea_orm_active_enums::PermissionEnum,
            sets, shared_quizes, shared_sets, users,
        },
        enums::error::*,
        models::set::ShareSetForUser,
    },
    chrono::Utc,
    sea_orm::{
        sea_query::OnConflict, ActiveModelTrait, ColumnTrait, Condition, EntityTrait, JoinType,
        ModelTrait, QueryFilter, QuerySelect, RelationTrait, Set,
    },
    std::sync::Arc,
    uuid::Uuid,
};

pub struct SetRepository {
    db: Arc<Database>,
}

impl SetRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    // Khi lấy toàn bộ set mà user create hoặc được share 
    // đối với create  
    pub async fn get_all_sets_of_user(&self, caller_id: Uuid) -> Result<Vec<sets::Model>> {
        let conn = self.db.get_connection().await;
        Sets::find().all(&conn).await.map_err(Error::QueryFailed)
    }

    pub async fn get_by_id(&self, caller_id: Uuid, set_id: Uuid) -> Result<sets::Model> {
        let conn = self.db.get_connection().await;
        
        // WHERE (owner_id = caller_id OR shared_sets.user_id = caller_id) 
        // AND is_delete = false
        let condition = Condition::all()
            .add(
                Condition::any().add(
                    sets::Column::OwnerId
                        .eq(caller_id)
                        .add(shared_sets::Column::UserId.eq(caller_id)),
                ),
            )
            .add(sets::Column::IsDeleted.eq(false));

        Sets::find_by_id(set_id)
            .filter(condition)
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

    pub async fn create_one(
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

    pub async fn update_one(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        public_or_not: Option<bool>,
    ) -> Result<Option<sets::Model>> {
        let conn = self.db.get_connection().await;

        let mut set: sets::ActiveModel = Sets::find_by_id(id)
            .filter(sets::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?
            .into();

        let mut updated = false;
        if let Some(name) = name {
            set.name = Set(name);
            updated = true;
        }
        if let Some(d) = description {
            set.description = Set(Some(d));
            updated = true;
        }
        if let Some(p) = public_or_not {
            set.public_or_not = Set(p);
            updated = true;
        }

        if updated {
            set.updated_at = Set(Utc::now().naive_utc());
            Ok(Some(set.update(&conn).await.map_err(Error::UpdateFailed)?))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_one(&self, id: Uuid) -> Result<()> {
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

    pub async fn is_owner(&self, id: Uuid, user_id: Uuid) -> Result<()> {
        let conn = self.db.get_connection().await;
        // check owner permission first
        let _ = Sets::find_by_id(id)
            .filter(sets::Column::OwnerId.eq(user_id))
            .filter(sets::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::PermissionDenied)?;

        Ok(())
    }

    pub async fn check_permission(
        &self,
        id: Uuid,
        user_id: Uuid,
        permission: PermissionEnum,
    ) -> Result<()> {
        let conn = self.db.get_connection().await;
        let _ = SharedSets::find()
            .filter(shared_sets::Column::SetId.eq(id))
            .filter(shared_sets::Column::UserId.eq(user_id))
            .filter(shared_sets::Column::Permission.eq(permission))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::PermissionDenied)?;

        Ok(())
    }

    pub async fn create_share_set(
        &self,
        set_id: Uuid,
        sharing_users: Vec<ShareSetForUser>,
    ) -> Result<Vec<shared_sets::Model>> {
        let conn = self.db.get_connection().await;

        let shared_sets = SharedSets::find()
            .filter(shared_quizes::Column::QuizId.eq(set_id))
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)?;

        // filter all shared user not in sharing users
        let sharing_user_ids = sharing_users
            .iter()
            .map(|u| u.user_id)
            .collect::<Vec<Uuid>>();
        let users_to_unshare = shared_sets
            .iter()
            .filter(|q| !sharing_user_ids.contains(&q.user_id))
            .map(|q| q.user_id)
            .collect::<Vec<Uuid>>();
        // delete all unshare users that had been shared before
        if !users_to_unshare.is_empty() {
            let deleted_shares = SharedSets::delete_many()
                .filter(shared_sets::Column::SetId.eq(set_id))
                .filter(shared_sets::Column::UserId.is_in(users_to_unshare))
                .exec_with_returning(&conn)
                .await
                .map_err(Error::DeleteFailed)?;

            debug!("users_to_unshare: {deleted_shares:?}");
        }

        let new_sharing_sets = sharing_users
            .into_iter()
            .map(|u| {
                let mut am = shared_sets::ActiveModel {
                    set_id: Set(set_id),
                    user_id: Set(u.user_id),
                    ..Default::default()
                };
                if let Some(p) = u.permission {
                    am.permission = Set(p);
                }
                am
            })
            .collect::<Vec<shared_sets::ActiveModel>>();
        let on_conflict = OnConflict::column(shared_quizes::Column::UserId)
            .update_column(shared_sets::Column::Permission)
            .to_owned();

        SharedSets::insert_many(new_sharing_sets)
            .on_conflict(on_conflict)
            .exec_with_returning_many(&conn)
            .await
            .map_err(Error::InsertFailed)
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
