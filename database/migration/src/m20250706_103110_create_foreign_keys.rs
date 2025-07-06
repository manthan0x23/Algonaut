use sea_orm_migration::prelude::*;

use crate::{
    m20250705_172030_create_user_table::User, m20250706_064538_room::Room,
    m20250706_075449_create_user_room_table::UserRoom,
    m20250706_075516_create_executions_table::Execution, m20250706_075536_create_chats_table::Chat,
    m20250706_075550_create_transactions_table::Transaction,
    m20250706_082017_create_tarsy_interactions_table::TarsyInteractions,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("room_created_by_fk")
                    .from(Room::Table, Room::CreatedBy)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("user_room_room_id_by_fk")
                    .from(UserRoom::Table, UserRoom::RoomId)
                    .to(Room::Table, Room::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("user_room_user_id_by_fk")
                    .from(UserRoom::Table, UserRoom::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("executions_room_id_by_fk")
                    .from(Execution::Table, Execution::RoomId)
                    .to(Room::Table, Room::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("executions_user_id_by_fk")
                    .from(Execution::Table, Execution::CreatedBy)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("chats_room_id_by_fk")
                    .from(Chat::Table, Chat::RoomId)
                    .to(Room::Table, Room::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("chats_user_id_by_fk")
                    .from(Chat::Table, Chat::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("transactions_user_id_by_fk")
                    .from(Transaction::Table, Transaction::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("tarsy_interactions_user_id_by_fk")
                    .from(TarsyInteractions::Table, TarsyInteractions::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("tarsy_interactions_room_id_by_fk")
                    .from(TarsyInteractions::Table, TarsyInteractions::RoomId)
                    .to(Room::Table, Room::Id) 
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(ForeignKey::drop().name("room_created_by_fk").to_owned())
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("user_room_room_id_by_fk")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("user_room_user_id_by_fk")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("executions_room_id_by_fk")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("executions_user_id_by_fk")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(ForeignKey::drop().name("chats_room_id_by_fk").to_owned())
            .await?;

        manager
            .drop_foreign_key(ForeignKey::drop().name("chats_user_id_by_fk").to_owned())
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("transactions_user_id_by_fk")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("tarsy_interactions_user_id_by_fk")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
