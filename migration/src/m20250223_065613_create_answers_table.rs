use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250223_065024_create_questions_table::Questions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Answers::Table)
                    .if_not_exists()
                    .col(pk_uuid(Answers::Id))
                    .col(text(Answers::Content)) // Changed from string() to text()
                    .col(boolean(Answers::IsCorrect))
                    .col(uuid(Answers::QuestionId)) // Changed to use uuid() directly
                    .col(timestamp(Answers::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Answers::UpdatedAt).default(Expr::current_timestamp()))
                    .col(boolean(Answers::IsDeleted).default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_answers_question_id")
                            .from(Answers::Table, Answers::QuestionId)
                            .to(Questions::Table, Questions::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Answers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Answers {
    Table,
    Id,
    Content,
    IsCorrect,
    QuestionId,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}
