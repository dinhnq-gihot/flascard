//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use {
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "quizes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub creator_id: Uuid,
    pub name: String,
    pub is_public: bool,
    pub question_counts: Json,
    pub is_published: bool,
    pub total_point: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub publish_at: Option<DateTime>,
    pub is_deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::quiz_questions::Entity")]
    QuizQuestions,
    #[sea_orm(has_many = "super::shared_quizes::Entity")]
    SharedQuizes,
    #[sea_orm(has_many = "super::tests::Entity")]
    Tests,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::CreatorId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    Users,
}

impl Related<super::quiz_questions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QuizQuestions.def()
    }
}

impl Related<super::shared_quizes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SharedQuizes.def()
    }
}

impl Related<super::tests::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tests.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        super::shared_quizes::Relation::Users.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::shared_quizes::Relation::Quizes.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
