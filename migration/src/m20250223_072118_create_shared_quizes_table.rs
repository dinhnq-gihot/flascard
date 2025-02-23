use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250223_061404_create_users_table::Users, m20250223_070735_create_quizes_table::Quizes,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SharedQuizes::Table)
                    .if_not_exists()
                    .col(uuid(SharedQuizes::QuizId))
                    .col(uuid(SharedQuizes::UserId))
                    .col(timestamp(SharedQuizes::SharedAt).default(Expr::current_timestamp()))
                    .primary_key(
                        Index::create()
                            .name("pk_user_quiz")
                            .col(SharedQuizes::QuizId)
                            .col(SharedQuizes::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_shared_quizes_quiz_id")
                            .from(SharedQuizes::Table, SharedQuizes::QuizId)
                            .to(Quizes::Table, Quizes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_shared_quizes_user_id")
                            .from(SharedQuizes::Table, SharedQuizes::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SharedQuizes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SharedQuizes {
    Table,
    QuizId,
    UserId,
    SharedAt,
}
