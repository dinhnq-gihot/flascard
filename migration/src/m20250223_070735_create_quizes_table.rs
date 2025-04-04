use {
    crate::{m20250223_061404_create_users_table::Users, m20250223_064318_create_sets_table::Sets},
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
                    .table(Quizes::Table)
                    .if_not_exists()
                    .col(pk_uuid(Quizes::Id).default(Expr::cust("uuid_generate_v4()")))
                    .col(uuid(Quizes::CreatorId))
                    .col(string(Quizes::Name))
                    .col(boolean(Quizes::IsPublic).default(false))
                    .col(json(Quizes::QuestionCounts))
                    .col(boolean(Quizes::IsPublished).default(false))
                    .col(unsigned(Quizes::TotalPoint).default(0))
                    .col(unsigned(Quizes::Duration).default(0))
                    .col(timestamp(Quizes::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Quizes::UpdatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_null(Quizes::PublishAt))
                    .col(boolean(Quizes::IsDeleted).default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_quizes_creator_id")
                            .from(Quizes::Table, Quizes::CreatorId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Quizes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Quizes {
    Table,
    Id,
    CreatorId,
    Name,
    IsPublic,
    QuestionCounts,
    IsPublished,
    TotalPoint,
    Duration,
    CreatedAt,
    UpdatedAt,
    PublishAt,
    IsDeleted,
}
