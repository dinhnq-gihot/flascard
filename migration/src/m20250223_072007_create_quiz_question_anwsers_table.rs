use {
    crate::m20250223_071935_create_quiz_questions_table::QuizQuestions,
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
                    .table(QuizQuestionAnswers::Table)
                    .if_not_exists()
                    .col(pk_uuid(QuizQuestionAnswers::Id).default(Expr::cust("uuid_generate_v4()")))
                    .col(uuid(QuizQuestionAnswers::QuizQuestionId))
                    .col(text(QuizQuestionAnswers::Content))
                    .col(boolean(QuizQuestionAnswers::IsAnswer))
                    .col(
                        timestamp(QuizQuestionAnswers::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(QuizQuestionAnswers::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(boolean(QuizQuestionAnswers::IsDeleted).default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_quiz_question_answers_to_quiz_question_id")
                            .from(
                                QuizQuestionAnswers::Table,
                                QuizQuestionAnswers::QuizQuestionId,
                            )
                            .to(QuizQuestions::Table, QuizQuestions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(QuizQuestionAnswers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum QuizQuestionAnswers {
    Table,
    Id,
    QuizQuestionId,
    Content,
    IsAnswer,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}
