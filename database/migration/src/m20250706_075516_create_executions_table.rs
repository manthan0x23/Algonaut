use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Execution::Table)
                    .if_not_exists()
                    .col(string(Execution::Id).primary_key().unique_key().not_null())
                    .col(string(Execution::Code))
                    .col(string(Execution::Input))
                    .col(string_null(Execution::Output))
                    .col(string(Execution::ExpectedOutput))
                    .col(string(Execution::CreatedBy).not_null())
                    .col(string(Execution::RoomId).not_null())
                    .col(string(Execution::Language))
                    .col(string(Execution::Status))
                    .col(string_null(Execution::Error))
                    .col(
                        timestamp(Execution::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Execution::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Execution {
    Table,
    Id,
    Code,
    Language,
    Input,
    Output,
    ExpectedOutput,
    Status,
    Error,
    CreatedBy,
    RoomId,
    CreatedAt,
}
