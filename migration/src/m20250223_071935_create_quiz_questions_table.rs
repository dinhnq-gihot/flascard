use {
    crate::{
        m20250223_065024_create_questions_table::{QuestionType, QuestionTypeEnum},
        m20250223_070735_create_quizes_table::Quizes,
    },
    sea_orm_migration::{prelude::*, schema::*, sea_orm::Iterable},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(QuizQuestions::Table)
                    .if_not_exists()
                    .col(pk_uuid(QuizQuestions::Id).default(Expr::cust("uuid_generate_v4()")))
                    .col(uuid(QuizQuestions::QuizId))
                    .col(text(QuizQuestions::QuestionContent))
                    .col(enumeration(
                        QuizQuestions::Type,
                        QuestionTypeEnum,
                        QuestionType::iter(),
                    ))
                    .col(uuid_null(QuizQuestions::NextQuestion))
                    .col(uuid_null(QuizQuestions::PreviousQuestion))
                    .col(timestamp(QuizQuestions::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(QuizQuestions::UpdatedAt).default(Expr::current_timestamp()))
                    .col(boolean(QuizQuestions::IsDeleted).default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_QuizQuestions_quiz_id")
                            .from(QuizQuestions::Table, QuizQuestions::QuizId)
                            .to(Quizes::Table, Quizes::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(QuizQuestions::Table, QuizQuestions::NextQuestion)
                            .to(QuizQuestions::Table, QuizQuestions::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(QuizQuestions::Table, QuizQuestions::PreviousQuestion)
                            .to(QuizQuestions::Table, QuizQuestions::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        let fk_quizes_start_question_id = TableForeignKey::new()
            .name("fk_quizes_start_question_id")
            .from_tbl(Quizes::Table)
            .from_col(Quizes::StartQuestion)
            .to_tbl(QuizQuestions::Table)
            .to_col(QuizQuestions::Id)
            .on_delete(ForeignKeyAction::SetNull)
            .to_owned();

        let fk_quizes_last_question_id = TableForeignKey::new()
            .name("fk_quizes_last_question_id")
            .from_tbl(Quizes::Table)
            .from_col(Quizes::LastQuestion)
            .to_tbl(QuizQuestions::Table)
            .to_col(QuizQuestions::Id)
            .on_delete(ForeignKeyAction::SetNull)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(Quizes::Table)
                    .add_foreign_key(&fk_quizes_start_question_id)
                    .add_foreign_key(&fk_quizes_last_question_id)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Quizes::Table)
                    .drop_foreign_key(Alias::new("fk_quizes_start_question_id"))
                    .drop_foreign_key(Alias::new("fk_quizes_last_question_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(QuizQuestions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum QuizQuestions {
    Table,
    Id,
    QuizId,
    QuestionId,
    QuestionContent,
    Type,
    NextQuestion,
    PreviousQuestion,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}
