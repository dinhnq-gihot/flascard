use {
    crate::{
        m20250223_065024_create_questions_table::{QuestionType, QuestionTypeEnum, Questions},
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
                    .col(uuid_null(QuizQuestions::SampleId))
                    .col(text(QuizQuestions::QuestionContent))
                    .col(enumeration(
                        QuizQuestions::Type,
                        QuestionTypeEnum,
                        QuestionType::iter(),
                    ))
                    .col(integer(QuizQuestions::Index))
                    .col(unsigned(QuizQuestions::Point))
                    .col(text_null(QuizQuestions::Explanation))
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
                            .name("fk_QuizQuestions_sample_id")
                            .from(QuizQuestions::Table, QuizQuestions::SampleId)
                            .to(Questions::Table, Questions::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
    SampleId,
    QuestionContent,
    Type,
    Index,
    Point,
    Explanation,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}
