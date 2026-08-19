use crate::config::data::AppConfig;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};

/// Acquire pooled database connection from sqlx.
pub async fn acquire_database_connection() -> anyhow::Result<Pool<Postgres>, Error> {
    tracing::info!("Performing connection to database.");

    let config = AppConfig::load();

    PgPoolOptions::new()
        .max_connections(32)
        .connect(&config.get_database_url())
        .await
}

/// Performing embedded migration. Trusted to be 100% hit-or-miss.
pub async fn perform_database_migration(pool_ref: &Pool<Postgres>) -> anyhow::Result<()> {
    tracing::info!("Performing migrations.");

    sqlx::migrate!().run(pool_ref).await?;

    Ok(())
}
