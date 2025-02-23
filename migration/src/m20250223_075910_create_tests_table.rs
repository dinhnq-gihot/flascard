use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250223_061404_create_users_table::Users, m20250223_070735_create_quizes_table::Quizes,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tests::Table)
                    .if_not_exists()
                    .col(pk_uuid(Tests::Id))
                    .col(uuid(Tests::QuizId))
                    .col(uuid(Tests::UserId))
                    .col(integer(Tests::Score))
                    .col(boolean(Tests::Submitted))
                    .col(timestamp(Tests::StartedAt))
                    .col(timestamp(Tests::SubmittedAt))
                    .col(unsigned(Tests::Duration)) // Duration in seconds
                    .col(unsigned(Tests::TotalQuestion))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tests_quiz_id")
                            .from(Tests::Table, Tests::QuizId)
                            .to(Quizes::Table, Quizes::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tests_user_id")
                            .from(Tests::Table, Tests::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tests::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Tests {
    Table,
    Id,
    QuizId,
    UserId,
    Score,
    Submitted,
    StartedAt,
    SubmittedAt,
    Duration,
    TotalQuestion,
}
