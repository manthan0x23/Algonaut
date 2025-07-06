use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chat::Table)
                    .if_not_exists()
                    .col(string(Chat::Id).primary_key().unique_key().not_null())
                    .col(string(Chat::Type).not_null())
                    .col(string_null(Chat::Text))
                    .col(string_null(Chat::File))
                    .col(string(Chat::RoomId).not_null())
                    .col(string(Chat::UserId).not_null())
                    .col(
                        timestamp(Chat::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chat::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Chat {
    Table,
    Id,
    Type,
    Text,
    File,
    RoomId,
    UserId,
    CreatedAt,
}
