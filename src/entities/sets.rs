//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub owner_id: Uuid,
    pub description: Option<String>,
    pub public_or_not: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub is_deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::questions::Entity")]
    Questions,
    #[sea_orm(has_many = "super::quizes::Entity")]
    Quizes,
    #[sea_orm(has_many = "super::shared_sets::Entity")]
    SharedSets,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::OwnerId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    Users,
}

impl Related<super::questions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Questions.def()
    }
}

impl Related<super::quizes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Quizes.def()
    }
}

impl Related<super::shared_sets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SharedSets.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        super::shared_sets::Relation::Users.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::shared_sets::Relation::Sets.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
