use std::env::var;

/// Uh... the config?
pub struct AppConfig {
    /// Discord BOT token.
    pub token: String,

    /// SQLite database connection url.
    pub database_url: String,

    /// Server ID of the private test server.
    pub test_server_id: Option<u64>,
}

impl AppConfig {
    /// Load configurations from environment variables.
    /// You usually want to call this *after* calling [`dotenvy::dotenv()`] method.
    pub fn load() -> Self {
        tracing::info!("Loading environment variables.");

        let default_db_url = if cfg!(test) {
            "sqlite::memory:"
        } else {
            "sqlite:nameless.db"
        };

        Self {
            token: var("TOKEN").expect("TOKEN must be set"),

            database_url: var("DATABASE_URL")
                .unwrap_or(default_db_url.to_string())
                .parse()
                .unwrap(),

            test_server_id: option_env!("TEST_SERVER_ID").map(|value| value.parse().unwrap()),
        }
    }
}
