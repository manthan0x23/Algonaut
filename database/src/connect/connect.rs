use sea_orm::{ConnectOptions, Database, DbConn};
use sea_orm_migration::MigratorTrait;
use std::time::Duration;

pub mod migration {
    pub use migration::Migrator;
}

/// Connects, runs migrations, and returns a connection pool with custom options.
pub async fn connect_and_migrate(database_url: &str) -> Result<DbConn, Box<dyn std::error::Error>> {
    // Build options, bump pool size and fail fast on acquire
    let mut opts = ConnectOptions::new(database_url.to_string());
    opts.max_connections(20) // up from default (e.g. 5)
        .acquire_timeout(Duration::from_secs(5)) // timeout after 5 s instead of 30 s
        .sqlx_logging(true); // optional: suppress SQLx logs

    // Connect
    let db = Database::connect(opts)
        .await
        .map_err(|e| format!("DB connect error: {}", e))?;

    // Run migrations
    migration::Migrator::up(&db, None)
        .await
        .map_err(|e| format!("DB migration error: {}", e))?;

    println!("✅ Migrated and connected to database");
    Ok(db)
}

/// Connects (without running migrations) with the same tuned pool options.
pub async fn connect(database_url: &str) -> Result<DbConn, Box<dyn std::error::Error>> {
    let mut opts = ConnectOptions::new(database_url.to_string());
    opts.max_connections(20)
        .acquire_timeout(Duration::from_secs(5))
        .sqlx_logging(true);

    let db = Database::connect(opts)
        .await
        .map_err(|e| format!("DB connect error: {}", e))?;

    println!("✅ Connected to database");
    Ok(db)
}
