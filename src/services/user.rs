use {
    crate::{
        db::db::Database,
        entities::{prelude::*, users},
        error::DbResult,
    },
    sea_orm::EntityTrait,
    std::sync::Arc,
};

pub struct UserService {
    db: Arc<Database>,
}

impl UserService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn get_all_users(&self) -> DbResult<Vec<users::Model>> {
        let conn = self.db.get_connection().await;
        let users = Users::find().all(&conn).await?;
        Ok(users)
    }
}
