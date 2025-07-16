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
                    .as_enum(UserRoomTypeEnum)
                    .values(UserRoomTypeVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(UserRoomStatusEnum)
                    .values(UserRoomStatusVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserRoom::Table)
                    .if_not_exists()
                    .col(string(UserRoom::Id).primary_key().not_null())
                    .col(string(UserRoom::UserId).not_null())
                    .col(string(UserRoom::RoomId).not_null())
                    .col(string(UserRoom::Type).custom(UserRoomTypeEnum).not_null())
                    .col(
                        string(UserRoom::Status)
                            .custom(UserRoomStatusEnum)
                            .not_null()
                            .default(UserRoomStatusVariants::Joined.to_string()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRoom::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserRoom {
    Table,
    Id,
    UserId,
    RoomId,
    Status,
    Type,
}

#[derive(Iden)]
pub struct UserRoomTypeEnum;

#[derive(Iden, EnumIter)]
pub enum UserRoomTypeVariants {
    #[iden = "viewer"]
    Viewer,

    #[iden = "editor"]
    Editor,

    #[iden = "creator"]
    Creator,
}

#[derive(Iden)]
pub struct UserRoomStatusEnum;

#[derive(Iden, EnumIter)]
pub enum UserRoomStatusVariants {
    #[iden = "active"]
    Active,

    #[iden = "left"]
    Left,

    #[iden = "joined"]
    Joined,
}
