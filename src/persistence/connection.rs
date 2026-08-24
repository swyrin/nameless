use crate::config::AppConfig;
use crate::nameless_types::NamelessConnection;
use sqlx::Error;

/// Acquire pooled database connection from sqlx.
pub async fn acquire_database_connection() -> Result<NamelessConnection, Error> {
    tracing::info!("Performing connection to database.");

    let config = AppConfig::load();

    NamelessConnection::connect(&config.database_url).await
}

/// Performing embedded migration. Trusted to be 100% hit-or-miss.
pub async fn perform_database_migration(pool_ref: &NamelessConnection) {
    tracing::info!("Performing migrations.");

    sqlx::migrate!()
        .run(pool_ref)
        .await
        .expect("Unable to perform database migration.");
}
