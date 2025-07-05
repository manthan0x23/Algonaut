use sea_orm::{Database, DbConn};
use sea_orm_migration::MigratorTrait;

pub mod migration {
    pub use migration::Migrator;
}

pub async fn connect_and_migrate(database_url: &str) -> Result<DbConn, Box<dyn std::error::Error>> {
    let db = Database::connect(database_url)
        .await
        .expect("Couldn't connect database");

    migration::Migrator::up(&db, None)
        .await
        .expect("Couldn't Migrate database");

    println!("Migrated and Connected to Database ");

    Ok(db)
}

pub async fn connect(database_url: &str) -> Result<DbConn, Box<dyn std::error::Error>> {
    let db = Database::connect(database_url)
        .await
        .expect("Couldn't connect database");

    println!("Connected to Database ");

    Ok(db)
}
