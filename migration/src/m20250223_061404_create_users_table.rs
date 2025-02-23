use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
    sea_orm::{EnumIter, Iterable},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(RoleEnum)
                    .values(Role::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_uuid(Users::Id))
                    .col(string(Users::Name))
                    .col(string(Users::Email))
                    .col(string(Users::Password))
                    .col(enumeration_null(Users::Role, RoleEnum, Role::iter()))
                    .col(timestamp(Users::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Users::UpdatedAt).default(Expr::current_timestamp()))
                    .col(boolean(Users::IsDeleted).default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Name,
    Email,
    Password,
    #[sea_orm(iden = "role")]
    Role,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}

#[derive(DeriveIden)]
struct RoleEnum;

#[derive(Iden, EnumIter)]
pub enum Role {
    #[iden = "Staff"]
    Staff,
    #[iden = "Users"]
    Users,
}
