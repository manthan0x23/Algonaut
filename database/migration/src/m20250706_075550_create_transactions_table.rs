use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Transaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transaction::Id)
                            .string()
                            .not_null()
                            .primary_key()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Transaction::UserId).string().not_null())
                    .col(ColumnDef::new(Transaction::Type).string().not_null())
                    .col(ColumnDef::new(Transaction::Status).string().not_null())
                    .col(ColumnDef::new(Transaction::Credit).decimal().not_null())
                    .col(ColumnDef::new(Transaction::Balance).decimal().not_null())
                    .col(ColumnDef::new(Transaction::Currency).string().not_null())
                    .col(ColumnDef::new(Transaction::Message).text())
                    .col(ColumnDef::new(Transaction::ReferenceId).string())
                    .col(ColumnDef::new(Transaction::Metadata).json())
                    .col(
                        ColumnDef::new(Transaction::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(ColumnDef::new(Transaction::ProcessedAt).timestamp())
                    .col(
                        ColumnDef::new(Transaction::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transaction::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Transaction {
    Table,
    Id,
    UserId,
    Type,
    Status,
    Credit,
    Balance,
    Currency,
    Message,
    ReferenceId,
    Metadata,
    CreatedAt,
    ProcessedAt,
    UpdatedAt,
}
