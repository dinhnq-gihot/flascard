use {
    crate::{
        db::db_connection::Database,
        debug,
        entities::{
            prelude::{Sets, SharedSets, Users},
            sets, shared_quizes, shared_sets, users,
        },
        enums::error::*,
        models::set::ShareSetForUser,
    },
    sea_orm::{
        sea_query::OnConflict, ColumnTrait, EntityTrait, JoinType, ModelTrait, QueryFilter,
        QuerySelect, RelationTrait, Set,
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

    pub async fn share_set(
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

        let sharing_user_ids = sharing_users
            .iter()
            .map(|u| u.user_id)
            .collect::<Vec<Uuid>>();
        let users_to_unshare = shared_sets
            .iter()
            .filter(|q| !sharing_user_ids.contains(&q.user_id))
            .map(|q| q.user_id)
            .collect::<Vec<Uuid>>();

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
