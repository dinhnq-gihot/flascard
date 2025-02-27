//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use {
    super::sea_orm_active_enums::RoleEnum,
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub role: RoleEnum,
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
    #[sea_orm(has_many = "super::sets::Entity")]
    Sets,
    #[sea_orm(has_many = "super::shared_quizes::Entity")]
    SharedQuizes,
    #[sea_orm(has_many = "super::shared_sets::Entity")]
    SharedSets,
    #[sea_orm(has_many = "super::tests::Entity")]
    Tests,
}

impl Related<super::questions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Questions.def()
    }
}

impl Related<super::shared_quizes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SharedQuizes.def()
    }
}

impl Related<super::shared_sets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SharedSets.def()
    }
}

impl Related<super::tests::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tests.def()
    }
}

impl Related<super::quizes::Entity> for Entity {
    fn to() -> RelationDef {
        super::shared_quizes::Relation::Quizes.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::shared_quizes::Relation::Users.def().rev())
    }
}

impl Related<super::sets::Entity> for Entity {
    fn to() -> RelationDef {
        super::shared_sets::Relation::Sets.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::shared_sets::Relation::Users.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
