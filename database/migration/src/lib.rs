pub use sea_orm_migration::prelude::*;

pub struct Migrator;
mod m20250705_172030_create_user_table;
mod m20250706_064538_room;
mod m20250706_075449_create_user_room_table;
mod m20250706_075516_create_executions_table;
mod m20250706_075536_create_chats_table;
mod m20250706_075550_create_transactions_table;
mod m20250706_082017_create_tarsy_interactions_table;
mod m20250706_103110_create_foreign_keys;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250705_172030_create_user_table::Migration),
            Box::new(m20250706_064538_room::Migration),
            Box::new(m20250706_075449_create_user_room_table::Migration),
            Box::new(m20250706_075516_create_executions_table::Migration),
            Box::new(m20250706_075536_create_chats_table::Migration),
            Box::new(m20250706_075550_create_transactions_table::Migration),
            Box::new(m20250706_082017_create_tarsy_interactions_table::Migration),
            Box::new(m20250706_103110_create_foreign_keys::Migration),
        ]
    }
}
