use {
    crate::{
        db::db_connection::Database,
        entities::{prelude::*, sea_orm_active_enums::RoleEnum, users},
        enums::error::{Error, Result},
        models::user::{RegisterUserRequest, UpdateUserRequest},
    },
    sea_orm::{
        ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, Set,
    },
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

    // Done ✅
    pub async fn get_all_users(&self) -> Result<Vec<users::Model>> {
        let conn = self.db.get_connection().await;
        Users::find()
            .filter(users::Column::IsDeleted.eq(false))
            .all(&conn)
            .await
            .map_err(Error::QueryFailed)
    }

    // Done ✅
    pub async fn get_by_id(&self, user_id: Uuid) -> Result<users::Model> {
        let conn = self.db.get_connection().await;
        Users::find_by_id(user_id)
            .filter(users::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)
    }

    // Done ✅
    pub async fn create_user(&self, payload: RegisterUserRequest) -> Result<users::Model> {
        let conn = self.db.get_connection().await;

        let RegisterUserRequest {
            email,
            password,
            name,
            role,
            avatar_url,
        } = payload;

        let new_user = users::ActiveModel {
            name: Set(name),
            email: Set(email),
            password: Set(password),
            role: Set(role),
            avatar_url: Set(avatar_url),
            ..Default::default()
        };

        users::Entity::insert(new_user)
            .exec_with_returning(&conn)
            .await
            .map_err(Error::InsertFailed)
    }

    // Done ✅
    pub async fn update_user(
        &self,
        user_id: Uuid,
        payload: UpdateUserRequest,
    ) -> Result<Option<users::Model>> {
        let conn = self.db.get_connection().await;

        let mut user: users::ActiveModel = Users::find_by_id(user_id)
            .filter(users::Column::IsDeleted.eq(false))
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)?
            .into();

        let UpdateUserRequest {
            name,
            email,
            password,
            role,
            avatar_url,
        } = payload;

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
        if let Some(avatar_url) = avatar_url {
            user.avatar_url = Set(Some(avatar_url));
            updated = true;
        }

        if updated {
            user.updated_at = Set(chrono::Utc::now().naive_utc());
            Ok(Some(user.update(&conn).await.map_err(Error::UpdateFailed)?))
        } else {
            Ok(None)
        }
    }

    // Done ✅
    pub async fn get_by_email(&self, email: String) -> Result<users::Model> {
        let conn = self.db.get_connection().await;
        Users::find()
            .filter(
                Condition::all()
                    .add(users::Column::Email.eq(email))
                    .add(users::Column::IsDeleted.eq(false)),
            )
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?
            .ok_or(Error::RecordNotFound)
    }

    // Done ✅
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

    pub async fn check_role(&self, user_id: Uuid, role: RoleEnum) -> Result<bool> {
        let conn = self.db.get_connection().await;

        let user = Users::find_by_id(user_id)
            .filter(
                Condition::all()
                    .add(users::Column::Role.eq(role))
                    .add(users::Column::IsDeleted.eq(true)),
            )
            .one(&conn)
            .await
            .map_err(Error::QueryFailed)?;

        Ok(user.is_some())
    }
}
