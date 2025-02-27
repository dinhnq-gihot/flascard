use {
    crate::{
        db::db::Database,
        entities::{prelude::*, sea_orm_active_enums::RoleEnum, users},
        enums::error::{Error, Result},
    },
    sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set},
    std::sync::Arc,
    uuid::Uuid,
};

pub struct UserService {
    db: Arc<Database>,
}

impl UserService {
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
    ) -> Result<users::Model> {
        let conn = self.db.get_connection().await;

        if let Some(email) = email {
            let existed_email_count = Users::find()
                .filter(users::Column::Email.eq(email))
                .count(&conn)
                .await
                .map_err(Error::QueryFailed)?;
            if existed_email_count > 0 {
                return Err(Error::UserAlreadyExists);
            }
        }

        Ok(())
    }
}
