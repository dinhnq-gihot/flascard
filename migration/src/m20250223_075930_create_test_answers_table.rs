use {
    crate::{
        m20250223_071935_create_quiz_questions_table::QuizQuestions,
        m20250223_072007_create_quiz_question_anwsers_table::QuizQuestionAnswers,
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
                    .table(TestAnswers::Table)
                    .if_not_exists()
                    .col(pk_uuid(TestAnswers::Id).default(Expr::cust("uuid_generate_v4()")))
                    .col(uuid(TestAnswers::TestId))
                    .col(uuid(TestAnswers::QuizQuestionId))
                    .col(text_null(TestAnswers::TextAnswer)) // Changed to text type
                    .col(uuid_null(TestAnswers::SelectedAnswerId))
                    .col(unsigned(TestAnswers::SpentTime).default(0))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_test_results_test_id")
                            .from(TestAnswers::Table, TestAnswers::TestId)
                            .to(Tests::Table, Tests::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_test_results_quiz_question_id")
                            .from(TestAnswers::Table, TestAnswers::QuizQuestionId)
                            .to(QuizQuestions::Table, QuizQuestions::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_test_results_quiz_question_answer_id")
                            .from(TestAnswers::Table, TestAnswers::SelectedAnswerId)
                            .to(QuizQuestionAnswers::Table, QuizQuestionAnswers::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TestAnswers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TestAnswers {
    Table,
    Id,
    TestId,
    QuizQuestionId,
    TextAnswer,
    SelectedAnswerId,
    SpentTime,
}
