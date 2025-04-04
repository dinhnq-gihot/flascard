use {
    crate::{
        m20250223_071935_create_quiz_questions_table::QuizQuestions,
        m20250223_075910_create_tests_table::Tests,
    },
    sea_orm_migration::{prelude::*, schema::*},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TestQuestionResults::Table)
                    .if_not_exists()
                    .col(pk_uuid(TestQuestionResults::Id).default(Expr::cust("uuid_generate_v4()")))
                    .col(uuid(TestQuestionResults::TestId))
                    .col(uuid(TestQuestionResults::QuizQuestionId))
                    .col(uuid(TestQuestionResults::Index))
                    .col(boolean_null(TestQuestionResults::IsCorrect))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_test_question_results__test_id")
                            .from(TestQuestionResults::Table, TestQuestionResults::TestId)
                            .to(Tests::Table, Tests::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_test_question_results__quiz_question_id")
                            .from(
                                TestQuestionResults::Table,
                                TestQuestionResults::QuizQuestionId,
                            )
                            .to(QuizQuestions::Table, QuizQuestions::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TestQuestionResults::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TestQuestionResults {
    Table,
    Id,
    TestId,
    QuizQuestionId,
    Index,
    IsCorrect,
}
