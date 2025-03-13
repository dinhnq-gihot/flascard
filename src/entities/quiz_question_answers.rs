//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use {
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "quiz_question_answers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub question_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub answer_content: String,
    pub is_answer: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub is_deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::questions::Entity",
        from = "Column::QuestionId",
        to = "super::questions::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Questions,
    #[sea_orm(
        belongs_to = "super::quizes::Entity",
        from = "Column::QuizId",
        to = "super::quizes::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    Quizes,
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

impl ActiveModelBehavior for ActiveModel {}
