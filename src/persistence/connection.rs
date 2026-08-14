use crate::config::data::AppConfig;
use crate::nameless_types::NamelessConnection;
use diesel_async::{AsyncConnection, AsyncMigrationHarness};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

const MIGRATION: EmbeddedMigrations = embed_migrations!();

pub async fn create_database_connection() -> NamelessConnection {
    let config = AppConfig::load();
    let db_url = config.get_database_url();

    NamelessConnection::establish(&db_url)
        .await
        .unwrap_or_else(|_| panic!("Unable to cook a connection to {}", db_url))
}

pub async fn perform_migration() {
    tracing::info!("Performing migrations.");

    let connection = create_database_connection().await;
    let mut migration_harness = AsyncMigrationHarness::new(connection);

    migration_harness
        .run_pending_migrations(MIGRATION)
        .expect("Migration execution failed.");
}
