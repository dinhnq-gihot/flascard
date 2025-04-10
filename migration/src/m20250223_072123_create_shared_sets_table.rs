use {
    crate::{m20250223_061404_create_users_table::Users, m20250223_064318_create_sets_table::Sets},
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
            .create_table(
                Table::create()
                    .table(SharedSets::Table)
                    .if_not_exists()
                    .col(uuid(SharedSets::SetId))
                    .col(uuid(SharedSets::UserId))
                    .col(timestamp(SharedSets::SharedAt).default(Expr::current_timestamp()))
                    .col(integer(SharedSets::Permission).default(0))
                    .primary_key(
                        Index::create()
                            .name("pk_user_set")
                            .col(SharedSets::SetId)
                            .col(SharedSets::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_shared_sets_set_id")
                            .from(SharedSets::Table, SharedSets::SetId)
                            .to(Sets::Table, Sets::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_shared_sets_user_id")
                            .from(SharedSets::Table, SharedSets::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SharedSets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SharedSets {
    Table,
    SetId,
    UserId,
    SharedAt,
    Permission,
}
