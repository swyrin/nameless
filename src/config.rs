use dotenv::{dotenv, option_dotenv};

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
    /// You usually want to call this *after* calling [`dotenvy::dotenv`] method.
    pub fn load() -> Self {
        tracing::info!("Loading environment variables.");

        Self {
            token: String::from(dotenv!("TOKEN")),
            database_url: String::from(
                option_dotenv!("DATABASE_URL").unwrap_or("sqlite:nameless.db"),
            ),
            test_server_id: option_dotenv!("TEST_SERVER_ID").map(|value| value.parse().unwrap()),
        }
    }
}
