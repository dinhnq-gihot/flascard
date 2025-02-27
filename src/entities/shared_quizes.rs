//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use {
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "shared_quizes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub quiz_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: Uuid,
    pub shared_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::quizes::Entity",
        from = "Column::QuizId",
        to = "super::quizes::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Quizes,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Users,
}

impl Related<super::quizes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Quizes.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
