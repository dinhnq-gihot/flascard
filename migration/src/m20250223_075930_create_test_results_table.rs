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
                    .table(Post::Table)
                    .if_not_exists()
                    .col(pk_auto(Post::Id))
                    .col(uuid(Post::TestId))
                    .col(uuid(Post::QuizQuestionId))
                    .col(text(Post::Result)) // Changed to text type
                    .col(boolean_null(Post::IsCorrect))
                    .col(unsigned(Post::SpentTime))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_test_results_test_id")
                            .from(Post::Table, Post::TestId)
                            .to(Tests::Table, Tests::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_test_results_quiz_question_id")
                            .from(Post::Table, Post::QuizQuestionId)
                            .to(QuizQuestions::Table, QuizQuestions::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    TestId,
    QuizQuestionId,
    Result,
    IsCorrect,
    SpentTime,
}
