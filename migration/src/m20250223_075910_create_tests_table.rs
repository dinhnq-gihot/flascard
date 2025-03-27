use {
    crate::{
        m20250223_061404_create_users_table::Users, m20250223_070735_create_quizes_table::Quizes,
    },
    sea_orm_migration::{
        prelude::{extension::postgres::Type, *},
        schema::*,
        sea_orm::{EnumIter, Iterable},
    },
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(StatusEnum)
                    .values(Status::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Tests::Table)
                    .if_not_exists()
                    .col(pk_uuid(Tests::Id).default(Expr::cust("uuid_generate_v4()")))
                    .col(uuid(Tests::QuizId))
                    .col(uuid(Tests::UserId))
                    .col(unsigned_null(Tests::Score))
                    .col(timestamp_null(Tests::StartedAt))
                    .col(timestamp_null(Tests::SubmittedAt))
                    .col(unsigned(Tests::Duration)) // Duration in seconds
                    .col(uuid(Tests::CurrentQuizQuestion))
                    .col(unsigned(Tests::RemainingTime))
                    .col(unsigned(Tests::CompletedQuestions))
                    .col(unsigned(Tests::TotalQuestion))
                    .col(enumeration(Tests::Status, StatusEnum, Status::iter()).default("NotStart"))
                    .col(timestamp(Tests::CreatedAt).default(Expr::current_timestamp()))
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
struct StatusEnum;

#[derive(Iden, EnumIter)]
pub enum Status {
    #[iden = "NotStart"]
    NotStart,
    #[iden = "InProgess"]
    InProgess,
    #[iden = "Submitted"]
    Submitted,
    #[iden = "Abandoned"]
    Abandoned,
}

#[derive(DeriveIden)]
pub enum Tests {
    Table,
    Id,
    QuizId,
    UserId,
    Score,
    StartedAt,
    SubmittedAt,
    Duration,
    CurrentQuizQuestion,
    RemainingTime,
    CompletedQuestions,
    TotalQuestion,
    #[sea_orm(iden = "status")]
    Status,
    CreatedAt,
}
