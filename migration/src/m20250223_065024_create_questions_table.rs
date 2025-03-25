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
            .create_type(
                Type::create()
                    .as_enum(QuestionTypeEnum)
                    .values(QuestionType::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Questions::Table)
                    .if_not_exists()
                    .col(pk_uuid(Questions::Id).default(Expr::cust("uuid_generate_v4()")))
                    .col(enumeration(
                        Questions::Type,
                        QuestionTypeEnum,
                        QuestionType::iter(),
                    ))
                    .col(string(Questions::Content))
                    .col(json(Questions::Answers))
                    .col(uuid(Questions::SetId))
                    .col(uuid(Questions::CreatorId))
                    .col(timestamp(Questions::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Questions::UpdatedAt).default(Expr::current_timestamp()))
                    .col(boolean(Questions::IsDeleted).default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_Questionss_set_id")
                            .from(Questions::Table, Questions::SetId)
                            .to(Sets::Table, Sets::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_Questionss_creator_id")
                            .from(Questions::Table, Questions::CreatorId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Questions::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(QuestionTypeEnum).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Questions {
    Table,
    Id,
    Type,
    Content,
    Answers,
    SetId,
    CreatorId,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}

#[derive(DeriveIden)]
pub struct QuestionTypeEnum;

#[derive(Iden, EnumIter)]
pub enum QuestionType {
    #[iden = "MultipleChoice"]
    MultipleChoice,
    #[iden = "CheckBoxes"]
    CheckBoxes,
    #[iden = "TextFill"]
    TextFill,
}
