use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250223_061404_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sets::Table)
                    .if_not_exists()
                    .col(pk_uuid(Sets::Id).default(Expr::cust("uuid_generate_v4()")))
                    // Add these new columns and foreign key
                    .col(uuid(Sets::OwnerId))
                    .col(string_null(Sets::Description))
                    .col(boolean(Sets::PublicOrNot).default(false))
                    .col(timestamp(Sets::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Sets::UpdatedAt).default(Expr::current_timestamp()))
                    .col(boolean(Sets::IsDeleted).default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sets_owner_id")
                            .from(Sets::Table, Sets::OwnerId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Sets {
    Table,
    Id,
    OwnerId,
    Description,
    PublicOrNot,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}
