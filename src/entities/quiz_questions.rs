//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use {
    super::sea_orm_active_enums::QuestionTypeEnum,
    sea_orm::entity::prelude::*,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "quiz_questions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub sample_id: Option<Uuid>,
    #[sea_orm(column_type = "Text")]
    pub question_content: String,
    pub r#type: QuestionTypeEnum,
    pub index: i32,
    pub point: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub explanation: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub is_deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::questions::Entity",
        from = "Column::SampleId",
        to = "super::questions::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Questions,
    #[sea_orm(has_many = "super::quiz_question_answers::Entity")]
    QuizQuestionAnswers,
    #[sea_orm(
        belongs_to = "super::quizes::Entity",
        from = "Column::QuizId",
        to = "super::quizes::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    Quizes,
    #[sea_orm(has_many = "super::test_answers::Entity")]
    TestAnswers,
    #[sea_orm(has_many = "super::test_question_results::Entity")]
    TestQuestionResults,
    #[sea_orm(has_many = "super::tests::Entity")]
    Tests,
}

impl Related<super::questions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Questions.def()
    }
}

impl Related<super::quiz_question_answers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QuizQuestionAnswers.def()
    }
}

impl Related<super::quizes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Quizes.def()
    }
}

impl Related<super::test_answers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TestAnswers.def()
    }
}

impl Related<super::test_question_results::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TestQuestionResults.def()
    }
}

impl Related<super::tests::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tests.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
