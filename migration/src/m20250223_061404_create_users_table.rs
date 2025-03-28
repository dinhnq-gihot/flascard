use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
    sea_orm::{DbBackend, EnumIter, Iterable, Statement},
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
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"".to_string(),
            ))
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_uuid(Users::Id).default(Expr::cust("uuid_generate_v4()")))
                    .col(string_uniq(Users::Name))
                    .col(string_uniq(Users::Email))
                    .col(string(Users::Password))
                    .col(enumeration(Users::Role, RoleEnum, Role::iter()).default("User"))
                    .col(string_null(Users::AvatarUrl))
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
            .await?;

        manager
            .drop_type(Type::drop().name(RoleEnum).to_owned())
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
    AvatarUrl,
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
    #[iden = "User"]
    User,
}
