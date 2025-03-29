use {
    crate::{
        db::db_connection::Database,
        entities::{
            prelude::{Sets, SharedSets},
            sets, shared_quizes, shared_sets,
        },
        enums::error::*,
        models::set::{
            AllSetsOfUserResponse, ShareSetForUser, SharedPermission, SharedSetsWithPermission,
        },
    },
    chrono::Utc,
    sea_orm::{
        sea_query::OnConflict, ActiveModelTrait, ColumnTrait, Condition, EntityTrait, JoinType,
        QueryFilter, QuerySelect, RelationTrait, Set,
    },
    serde_json::Value as JsonValue,
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
    // Done ✅
    pub async fn get_all_sets_of_user(&self, caller_id: Uuid) -> Result<AllSetsOfUserResponse> {
        let conn = self.db.get_connection().await;

        // Lấy toàn bộ set sở hữu
        let own_sets = Sets::find()
            .filter(
                Condition::all()
                    .add(sets::Column::OwnerId.eq(caller_id))
                    .add(sets::Column::IsDeleted.eq(false)),
            )
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)?;

        // Lấy toàn bộ set được share mà chưa public
        let shared_sets = Sets::find()
            .column(shared_sets::Column::Permission)
            .join(JoinType::InnerJoin, sets::Relation::SharedSets.def())
            .filter(
                Condition::all()
                    .add(shared_sets::Column::UserId.eq(caller_id))
                    .add(sets::Column::PublicOrNot.eq(false))
                    .add(sets::Column::IsDeleted.eq(false)),
            )
            .into_tuple::<(JsonValue, i32)>()
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .into_iter()
            .map(|s| {
                SharedSetsWithPermission {
                    set: serde_json::from_value::<sets::Model>(s.0).unwrap(),
                    permission: s.1.into(),
                }
            })
            .collect::<Vec<_>>();

        // lấy toàn bộ set public mà không phải user sở hữu
        let public_sets = Sets::find()
            .filter(
                Condition::all()
                    .add(sets::Column::OwnerId.ne(caller_id))
                    .add(sets::Column::PublicOrNot.eq(true))
                    .add(sets::Column::IsDeleted.eq(false)),
            )
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)?;

        Ok(AllSetsOfUserResponse {
            own_sets,
            shared_sets,
            public_sets,
        })
    }

    // Done ✅
    pub async fn get_by_id(&self, caller_id: Uuid, set_id: Uuid) -> Result<sets::Model> {
        let conn = self.db.get_connection().await;

        // WHERE (owner_id = caller_id OR shared_sets.user_id = caller_id OR
        // public_or_not = true) AND is_delete = false
        let condition = Condition::all()
            .add(
                Condition::any().add(
                    sets::Column::OwnerId
                        .eq(caller_id)
                        .add(shared_sets::Column::UserId.eq(caller_id))
                        .add(sets::Column::PublicOrNot.eq(true)),
                ),
            )
            .add(sets::Column::IsDeleted.eq(false));

        Sets::find_by_id(set_id)
            .join(JoinType::LeftJoin, sets::Relation::SharedSets.def())
            .filter(condition)
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)
    }

    // pub async fn get_by_owner_id(&self, owner_id: Uuid) ->
    // Result<Vec<sets::Model>> {     let conn = self.db.get_connection().await;
    //     Sets::find()
    //         .filter(sets::Column::OwnerId.eq(owner_id))
    //         .filter(sets::Column::IsDeleted.eq(false))
    //         .all(&conn)
    //         .await
    //         .map_err(Error::QueryFailed)
    // }

    // Done ✅
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

    // Done ✅
    pub async fn update_one(
        &self,
        set_id: Uuid,
        name: Option<String>,
        description: Option<String>,
        public_or_not: Option<bool>,
        caller_id: Uuid,
    ) -> Result<Option<sets::Model>> {
        let conn = self.db.get_connection().await;
        let mut set: sets::ActiveModel = Sets::find_by_id(set_id)
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
            set.latest_updater_id = Set(Some(caller_id));

            Ok(Some(set.update(&conn).await.map_err(Error::UpdateFailed)?))
        } else {
            Ok(None)
        }
    }

    // Done ✅
    pub async fn delete_one(&self, set_id: Uuid) -> Result<()> {
        let conn = self.db.get_connection().await;
        let mut set: sets::ActiveModel = Sets::find_by_id(set_id)
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

    // Done ✅
    pub async fn is_owner(&self, id: Uuid, user_id: Uuid) -> Result<bool> {
        let conn = self.db.get_connection().await;

        // check owner permission first
        let res = Sets::find_by_id(id)
            .filter(
                Condition::all()
                    .add(sets::Column::OwnerId.eq(user_id))
                    .add(sets::Column::IsDeleted.eq(false)),
            )
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?;

        Ok(res.is_some())
    }

    // Done ✅
    pub async fn check_share_permission(
        &self,
        set_id: Uuid,
        user_id: Uuid,
        permission: SharedPermission,
    ) -> Result<bool> {
        let conn = self.db.get_connection().await;

        // WHERE shared_sets.set_id = set_id AND shared_sets.user_id = user_id AND
        // shared_sets.permission = permission
        let condition = Condition::all()
            .add(shared_sets::Column::SetId.eq(set_id))
            .add(shared_sets::Column::UserId.eq(user_id))
            .add(shared_sets::Column::Permission.gte(permission as i32));

        let res = SharedSets::find()
            .filter(condition)
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?;

        Ok(res.is_some())
    }

    // Done ✅
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
            let condition = Condition::all()
                .add(shared_sets::Column::SetId.eq(set_id))
                .add(shared_sets::Column::UserId.is_in(users_to_unshare));

            let deleted_shares = SharedSets::delete_many()
                .filter(condition)
                .exec_with_returning(&conn)
                .await
                .map_err(Error::DeleteFailed)?;

            println!("users_to_unshare: {deleted_shares:#?}");
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
                    am.permission = Set(p as i32);
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

    // pub async fn get_shared_set(&self, set_id: Uuid, user_id: Uuid) ->
    // Result<Option<sets::Model>> {     let conn =
    // self.db.get_connection().await;

    //     Sets::find()
    //         .join(JoinType::InnerJoin, sets::Relation::SharedSets.def())
    //         .filter(shared_sets::Column::UserId.eq(user_id))
    //         .filter(shared_sets::Column::UserId.eq(set_id))
    //         .filter(sets::Column::IsDeleted.eq(false))
    //         .one(&conn)
    //         .await
    //         .map_err(Error::QueryFailed)
    // }

    // pub async fn get_all_shared_users_of_set(&self, set_id: Uuid) ->
    // Result<Vec<users::Model>> {     let conn =
    // self.db.get_connection().await;

    //     Users::find()
    //         .join(JoinType::InnerJoin, users::Relation::SharedSets.def())
    //         .filter(shared_sets::Column::SetId.eq(set_id))
    //         .filter(users::Column::IsDeleted.eq(false))
    //         .all(&conn)
    //         .await
    //         .map_err(Error::QueryFailed)
    // }

    // pub async fn get_all_shared_sets_of_user(&self, user_id: Uuid) ->
    // Result<Vec<sets::Model>> {     let conn = self.db.get_connection().await;

    //     let user = Users::find_by_id(user_id)
    //         .one(&conn)
    //         .await
    //         .map_err(Error::QueryFailed)?
    //         .ok_or(Error::RecordNotFound)?;

    //     user.find_related(Sets)
    //         .filter(sets::Column::IsDeleted.eq(false))
    //         .all(&conn)
    //         .await
    //         .map_err(Error::QueryFailed)

    //     // Sets::find()
    //     //     .join(JoinType::InnerJoin, sets::Relation::SharedSets.def())
    //     //     .filter(shared_sets::Column::UserId.eq(user_id))
    //     //     .filter(sets::Column::IsDeleted.eq(false))
    //     //     .all(&conn)
    //     //     .await
    //     //     .map_err(Error::QueryFailed)
    // }
}
