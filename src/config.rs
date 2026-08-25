use poise::serenity_prelude::GuildId;
use std::env::var;

/// Uh... the config?
pub struct AppConfig {
    /// Discord BOT token.
    pub token: String,

    /// SQLite database connection url.
    pub database_url: String,

    /// Server ID of the private test server.
    pub test_server_id: Option<GuildId>,
}

impl AppConfig {
    /// Load configurations from environment variables.
    /// You usually want to call this *after* calling [`dotenvy::dotenv()`] method.
    pub fn load() -> Self {
        tracing::info!("Loading environment variables.");

        Self {
            token: var("TOKEN").expect("TOKEN must be set"),

            database_url: var("DATABASE_URL")
                .unwrap_or("sqlite:nameless.db".to_string())
                .to_string(),

            test_server_id: match var("TEST_SERVER_ID") {
                Ok(v) => Some(v.parse::<u64>().unwrap().into()),
                Err(_) => None,
            },
        }
    }
}
