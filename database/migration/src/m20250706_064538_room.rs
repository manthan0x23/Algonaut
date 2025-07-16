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
                    .as_enum(RoomScopeTypeEnum)
                    .values(RoomScopeTypeVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Room::Table)
                    .if_not_exists()
                    .col(string(Room::Id).primary_key().unique_key().not_null())
                    .col(string(Room::Alias).not_null())
                    .col(string(Room::Objective).not_null())
                    .col(integer(Room::Capacity).default(40).not_null())
                    .col(
                        enumeration(
                            Room::EditorsScopeType,
                            RoomScopeTypeEnum,
                            RoomScopeTypeVariants::iter(),
                        )
                        .custom(RoomScopeTypeEnum)
                        .not_null()
                        .default(RoomScopeTypeVariants::Strict.to_string()),
                    )
                    .col(string_null(Room::Code).null())
                    .col(string_null(Room::CodeLanguage))
                    .col(
                        array(Room::AllowedViewers, ColumnType::String(StringLen::Max))
                            .not_null()
                            .default("{}"),
                    )
                    .col(
                        enumeration(
                            Room::ViewersScopeType,
                            RoomScopeTypeEnum,
                            RoomScopeTypeVariants::iter(),
                        )
                        .custom(RoomScopeTypeEnum)
                        .default(RoomScopeTypeVariants::Open.to_string()),
                    )
                    .col(
                        array(Room::AllowedEditors, ColumnType::String(StringLen::Max))
                            .not_null()
                            .default("{}"),
                    )
                    .col(string(Room::CreatedBy).not_null())
                    .col(timestamp(Room::CreatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Room::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Room {
    Table,
    Id,
    Alias,
    Objective,
    Code,
    CodeLanguage,
    Capacity,
    AllowedEditors,
    EditorsScopeType,
    AllowedViewers,
    ViewersScopeType,
    CreatedBy,
    CreatedAt,
}

#[derive(Iden)]
pub struct RoomScopeTypeEnum;

#[derive(Iden, EnumIter)]
pub enum RoomScopeTypeVariants {
    #[iden = "open"]
    Open,

    #[iden = "strict"]
    Strict,
}
