use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20250223_061404_create_users_table::Users, m20250223_064318_create_sets_table::Sets};

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
                    .col(pk_uuid(Quizes::Id))
                    .col(uuid(Quizes::SetId))
                    .col(uuid(Quizes::CreatorId))
                    .col(boolean(Quizes::PublicOrNot).default(false))
                    .col(json(Quizes::QuestionCounts))
                    .col(boolean(Quizes::IsPublished).default(false))
                    .col(timestamp(Quizes::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Quizes::UpdatedAt).default(Expr::current_timestamp()))
                    .col(boolean(Quizes::IsDeleted).default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_quizes_set_id")
                            .from(Quizes::Table, Quizes::SetId)
                            .to(Sets::Table, Sets::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
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
    SetId,
    CreatorId,
    PublicOrNot,
    QuestionCounts,
    IsPublished,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}
