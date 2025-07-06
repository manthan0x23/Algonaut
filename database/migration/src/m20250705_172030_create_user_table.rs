use common::id::short_id;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(string(User::Id).primary_key().not_null().unique_key())
                    .col(string(User::Email).not_null().unique_key())
                    .col(string_null(User::Name))
                    .col(string_null(User::AvatarUrl))
                    .col(timestamp(User::CreatedAt).default(Expr::current_timestamp()))
                    .col(big_integer(User::Credits).default(Expr::value(0)))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Name,
    Email,
    AvatarUrl,
    CreatedAt,
    Credits,
}
