use crate::config::AppConfig;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

/// Acquire database connection.
pub async fn acquire_database_connection() -> Result<DatabaseConnection, DbErr> {
    tracing::info!("Performing connection to database.");

    let config = AppConfig::load();
    let mut opt = ConnectOptions::new(&config.database_url);
    opt.sqlx_logging(false);

    Database::connect(opt).await
}

/// Performing embedded migration. 100% hit-or-miss.
pub async fn perform_database_migration(conn: &DatabaseConnection) {
    tracing::info!("Performing migrations.");

    conn.get_schema_registry("nameless_ng::*")
        .sync(conn)
        .await
        .expect("Failure in database schema migration.");

    tracing::info!("Done performing migrations.");
}
