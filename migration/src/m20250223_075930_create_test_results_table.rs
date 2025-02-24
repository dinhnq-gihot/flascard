use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250223_071935_create_quiz_questions_table::QuizQuestions,
    m20250223_075910_create_tests_table::Tests,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TestResults::Table)
                    .if_not_exists()
                    .col(pk_auto(TestResults::Id))
                    .col(uuid(TestResults::TestId))
                    .col(uuid(TestResults::QuizQuestionId))
                    .col(text_null(TestResults::Result)) // Changed to text type
                    .col(boolean_null(TestResults::IsCorrect))
                    .col(unsigned(TestResults::SpentTime))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_test_results_test_id")
                            .from(TestResults::Table, TestResults::TestId)
                            .to(Tests::Table, Tests::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_test_results_quiz_question_id")
                            .from(TestResults::Table, TestResults::QuizQuestionId)
                            .to(QuizQuestions::Table, QuizQuestions::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TestResults::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TestResults {
    Table,
    Id,
    TestId,
    QuizQuestionId,
    Result,
    IsCorrect,
    SpentTime,
}
