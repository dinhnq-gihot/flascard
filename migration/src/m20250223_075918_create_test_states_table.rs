use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250223_075910_create_tests_table::Tests;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TestStates::Table)
                    .if_not_exists()
                    .col(pk_auto(TestStates::Id))
                    .col(uuid(TestStates::TestId))
                    .col(unsigned(TestStates::CurrentQuizQuestion))
                    .col(unsigned(TestStates::RemainingTime))
                    .col(unsigned(TestStates::CompletedQuestions))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_test_states_test_id")
                            .from(TestStates::Table, TestStates::TestId)
                            .to(Tests::Table, Tests::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TestStates::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TestStates {
    Table,
    Id,
    TestId,
    CurrentQuizQuestion,
    RemainingTime,
    CompletedQuestions,
}
