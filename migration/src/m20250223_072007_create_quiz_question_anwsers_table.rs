use {
    crate::{
        m20250223_065024_create_questions_table::Questions,
        m20250223_070735_create_quizes_table::Quizes,
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
                    .table(QuizQuestionAnswers::Table)
                    .if_not_exists()
                    .col(pk_uuid(QuizQuestionAnswers::Id).default(Expr::cust("uuid_generate_v4()")))
                    .col(uuid(QuizQuestionAnswers::QuizId))
                    .col(uuid(QuizQuestionAnswers::QuestionId))
                    .col(text(QuizQuestionAnswers::AnswerContent))
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
                            .name("fk_quiz_question_answers_quiz_id")
                            .from(QuizQuestionAnswers::Table, QuizQuestionAnswers::QuizId)
                            .to(Quizes::Table, Quizes::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_quiz_question_answers_question_id")
                            .from(QuizQuestionAnswers::Table, QuizQuestionAnswers::QuestionId)
                            .to(Questions::Table, Questions::Id)
                            .on_delete(ForeignKeyAction::Restrict),
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
    QuizId,
    QuestionId,
    AnswerContent,
    IsAnswer,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}
