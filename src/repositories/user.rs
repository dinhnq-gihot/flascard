use {
    crate::{
        db::db_connection::Database,
        entities::{prelude::*, sea_orm_active_enums::RoleEnum, users},
        enums::error::{Error, Result},
    },
    sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set},
    std::sync::Arc,
    uuid::Uuid,
};

pub struct UserRepository {
    db: Arc<Database>,
}

impl UserRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_all_users(&self) -> Result<Vec<users::Model>> {
        let conn = self.db.get_connection().await;
        let users = Users::find().all(&conn).await.map_err(Error::QueryFailed)?;
        Ok(users)
    }

    pub async fn create_user(
        &self,
        email: String,
        password: String,
        name: String,
        role: RoleEnum,
    ) -> Result<users::Model> {
        let conn = self.db.get_connection().await;

        let new_user = users::ActiveModel {
            name: Set(name),
            email: Set(email),
            password: Set(password),
            role: Set(role),
            ..Default::default()
        };

        let res = users::Entity::insert(new_user)
            .exec_with_returning(&conn)
            .await
            .map_err(Error::InsertFailed)?;

        Ok(res)
    }

    pub async fn update_user(
        &self,
        id: Uuid,
        name: Option<String>,
        email: Option<String>,
        password: Option<String>,
        role: Option<RoleEnum>,
    ) -> Result<Option<users::Model>> {
        let conn = self.db.get_connection().await;

        let mut user: users::ActiveModel = Users::find_by_id(id)
            .filter(users::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?
            .into();

        let mut updated = false;
        if let Some(email) = email {
            let existed_email_count = Users::find()
                .filter(users::Column::Email.eq(&email))
                .filter(users::Column::IsDeleted.eq(false))
                .count(&conn)
                .await
                .map_err(Error::QueryFailed)?;
            if existed_email_count > 0 {
                return Err(Error::UserAlreadyExists);
            } else {
                user.email = Set(email);
                updated = true;
            }
        }
        if let Some(name) = name {
            user.name = Set(name);
            updated = true;
        }
        if let Some(password) = password {
            user.password = Set(password);
            updated = true;
        }
        if let Some(role) = role {
            user.role = Set(role);
            updated = true;
        }

        if updated {
            user.updated_at = Set(chrono::Utc::now().naive_utc());
            Ok(Some(user.update(&conn).await.map_err(Error::UpdateFailed)?))
        } else {
            Ok(None)
        }
    }

    pub async fn get_by_email(&self, email: String) -> Result<Option<users::Model>> {
        let conn = self.db.get_connection().await;
        let user = Users::find()
            .filter(users::Column::Email.eq(email))
            .filter(users::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?;

        Ok(user)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<()> {
        let conn = self.db.get_connection().await;
        let mut user: users::ActiveModel = Users::find_by_id(id)
            .filter(users::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?
            .into();
        user.is_deleted = Set(true);
        user.update(&conn).await.map_err(Error::DeleteFailed)?;

        Ok(())
    }
}
