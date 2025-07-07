use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TarsyInteractions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TarsyInteractions::Id)
                            .string()
                            .not_null()
                            .primary_key()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(TarsyInteractions::UserId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TarsyInteractions::RoomId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TarsyInteractions::Prompt).text().not_null())
                    .col(ColumnDef::new(TarsyInteractions::Output).text())
                    .col(
                        ColumnDef::new(TarsyInteractions::Tokens)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TarsyInteractions::PromptTokens).integer())
                    .col(ColumnDef::new(TarsyInteractions::CompletionTokens).integer())
                    .col(ColumnDef::new(TarsyInteractions::Model).string().not_null())
                    .col(
                        ColumnDef::new(TarsyInteractions::Status)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TarsyInteractions::LatencyMs).integer())
                    .col(
                        ColumnDef::new(TarsyInteractions::CreditsCost)
                            .integer()
                            .not_null()
                            .default(10),
                    )
                    .col(ColumnDef::new(TarsyInteractions::Metadata).json())
                    .col(ColumnDef::new(TarsyInteractions::IpAddress).string())
                    .col(ColumnDef::new(TarsyInteractions::ReferenceId).string())
                    .col(
                        ColumnDef::new(TarsyInteractions::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(TarsyInteractions::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(ColumnDef::new(TarsyInteractions::ProcessedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TarsyInteractions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TarsyInteractions {
    Table,
    Id,
    UserId,
    RoomId,
    Prompt,
    Output,
    Tokens,
    PromptTokens,
    CompletionTokens,
    Model,
    Status,
    LatencyMs,
    CreditsCost,
    Metadata,
    IpAddress,
    ReferenceId,
    CreatedAt,
    UpdatedAt,
    ProcessedAt,
}
